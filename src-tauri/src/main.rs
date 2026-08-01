#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use btleplug::platform::Peripheral as PlatformPeripheral;
use loose_idasen::BtError;
use serde::Serialize;
use tauri::{
    async_runtime::block_on, tray::TrayIconBuilder, AppHandle, Manager, WebviewUrl, WebviewWindow,
    WebviewWindowBuilder,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

mod config_utils;
mod desk_mutex;
mod loose_idasen;
mod tray_utils;

const MAIN_TRAY_ID: &str = "main-tray";
const ERROR_ROUTE: &str = "/error";
const ERROR_DESCRIPTION: &str = "Either try reconnecting with that desk from your system and relaunch Trayasen, or click the button below to run the setup again.";

pub struct TauriSharedDesk(Mutex<Result<PlatformPeripheral, BtError>>);

#[derive(Serialize)]
struct ErrorWindowState<'a> {
    title: String,
    description: String,
    desk_name: &'a str,
    error: &'a str,
}

fn json_for_initialization_script(value: &impl Serialize) -> String {
    // JSON string escaping handles quotes, backslashes and control characters. Escaping the
    // HTML-sensitive characters and JavaScript line separators additionally keeps payloads inert
    // if this script is ever moved into an inline script context.
    serde_json::to_string(value)
        .expect("initialization state should be JSON serializable")
        .replace('&', "\\u0026")
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029")
}

pub(crate) fn route_initialization_script(route: &str) -> String {
    let route = json_for_initialization_script(&route);
    format!("history.replaceState({{}}, '', {route});")
}

fn error_state_initialization_script(
    title: String,
    description: &str,
    desk_name: &str,
    error: &str,
) -> String {
    let state = ErrorWindowState {
        title,
        description: description.to_string(),
        desk_name,
        error,
    };
    let state = json_for_initialization_script(&state);
    let route = json_for_initialization_script(&ERROR_ROUTE);

    format!("window.stateWorkaround = {state};\nhistory.replaceState({{}}, '', {route});")
}

fn connection_error_initialization_script(desk_name: &str, error: &str) -> String {
    error_state_initialization_script(
        format!("The app was not able to connect to your saved desk with name: `{desk_name}`."),
        ERROR_DESCRIPTION,
        desk_name,
        error,
    )
}

fn config_recovery_initialization_script(error: &str) -> String {
    error_state_initialization_script(
        "Trayasen could not read your configuration.".to_string(),
        "The original config was left unchanged. Reset it below to create a fresh config and restart Trayasen.",
        "",
        error,
    )
}

// Whether a system should have custom decorations or not
#[tauri::command]
fn has_custom_decorations() -> bool {
    cfg!(windows)
}

pub trait WindowInitUtils {
    fn init_trayasen(self, title: &str, init_script: Option<&str>) -> tauri::Result<WebviewWindow>;
}

impl<'a, M> WindowInitUtils for WebviewWindowBuilder<'a, tauri::Wry, M>
where
    M: Manager<tauri::Wry>,
{
    fn init_trayasen(self, title: &str, init_script: Option<&str>) -> tauri::Result<WebviewWindow> {
        // We want to replace borders only on Windows, as on macOS they are pretty enough, and on Linux custom shadows are unsupported.
        let mut window_builder = if has_custom_decorations() {
            self.inner_size(1280.0, 720.0)
                .title(title)
                .always_on_top(true)
                .decorations(false)
                .shadow(true)
        } else {
            self.inner_size(1280.0, 720.0)
                .title(title)
                .always_on_top(true)
        };

        if let Some(init_script) = init_script {
            window_builder = window_builder.initialization_script(init_script);
        }

        window_builder.build()
    }
}

fn open_main_window(app: &AppHandle, title: &str, init_script: Option<&str>) {
    match WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
        .init_trayasen(title, init_script)
    {
        Ok(window) => {
            if let Err(error) = window.show() {
                eprintln!("Could not show `{title}` window: {error}");
            }
            if let Err(error) = window.set_focus() {
                eprintln!("Could not focus `{title}` window: {error}");
            }
        }
        Err(error) => eprintln!("Could not create `{title}` window: {error}"),
    }
}

fn register_position_shortcut(
    app_handle: &AppHandle,
    position: &config_utils::Position,
    desk: PlatformPeripheral,
) {
    let Some(shortcut) = position.shortcut.as_deref() else {
        return;
    };
    if shortcut.is_empty() {
        return;
    }

    let position_name = position.name.clone();
    let position_name_for_callback = position_name.clone();
    let target = position.value;
    if let Err(error) =
        app_handle
            .global_shortcut()
            .on_shortcut(shortcut, move |_app_handle, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    block_on(async {
                        if let Err(error) = loose_idasen::move_to_target(&desk, target).await {
                            eprintln!(
                                "Failed to move desk to position `{position_name_for_callback}` from shortcut: {error}"
                            );
                        }
                    });
                }
            })
    {
        eprintln!(
            "Failed to register global shortcut `{shortcut}` for position `{position_name}`: {error}"
        );
    }
}

#[tauri::command]
fn create_new_elem(
    app_handle: tauri::AppHandle,
    name: &str,
    value: u16,
    shortcutvalue: Option<String>,
) -> Result<String, String> {
    let mut config = config_utils::get_config(app_handle.clone())?;

    println!("shortcut_acc: {:?}", shortcutvalue);

    let is_duplicate = config.saved_positions.iter().find(|elem| elem.name == name);
    match is_duplicate {
        Some(_) => {
            // Duplicate found
            Ok("duplicate".to_string())
        }
        None => {
            // No duplicate
            let position = config_utils::Position {
                name: name.to_string(),
                value,
                shortcut: shortcutvalue,
            };
            config.saved_positions.push(position.clone());
            config_utils::update_config(&app_handle, &config)?;

            if position
                .shortcut
                .as_deref()
                .is_some_and(|key| !key.is_empty())
            {
                match desk_mutex::get_desk_from_app_state(&app_handle) {
                    Ok(desk) => register_position_shortcut(&app_handle, &position, desk),
                    Err(error) => eprintln!(
                        "Could not register shortcut for position `{}`: {error}",
                        position.name
                    ),
                };
            }

            Ok("success".to_string())
        }
    }
}

fn persist_after_success<T>(
    connection: Result<T, String>,
    persist: impl FnOnce() -> Result<(), String>,
) -> Result<T, String> {
    let connected = connection?;
    persist()?;
    Ok(connected)
}

/// Provided a name, will connect to a desk with this name - after this step, desk actually becomes usable
#[tauri::command]
async fn connect_to_desk_by_name(app_handle: tauri::AppHandle, name: String) -> Result<(), String> {
    println!("connecting to desk with name: {}", name);
    let instantiated_desk = app_handle.state::<TauriSharedDesk>();
    println!("with desk!...");
    let connection = loose_idasen::connect_to_desk_by_name_internal(name.clone())
        .await
        .map_err(|error| error.to_string());
    println!("after cached desk...");
    let cached_desk = persist_after_success(connection, || {
        config_utils::save_local_name(&app_handle, name)
    })?;

    desk_mutex::assign_desk_to_mutex(&instantiated_desk, Ok(cached_desk));
    println!("Successfuly connected to desk from frontend");
    Ok(())
}

fn handle_tray_menu_event(app: &AppHandle, id: &str) {
    match id {
        config_utils::QUIT_ID => tray_utils::handle_exit_menu_click(),
        config_utils::ABOUT_ID => tray_utils::handle_about_menu_click(app),
        config_utils::ADD_POSITION_ID => tray_utils::handle_new_position_menu_click(app),
        config_utils::MANAGE_POSITIONS_ID => tray_utils::handle_manage_positions_menu_click(app),
        remaining_id => {
            let Some(position_name) = config_utils::position_name_from_menu_id(remaining_id) else {
                eprintln!("Ignoring unknown tray menu item `{remaining_id}`");
                return;
            };
            let config = match config_utils::get_config(app.clone()) {
                Ok(config) => config,
                Err(error) => {
                    eprintln!("Could not load config for tray position `{position_name}`: {error}");
                    return;
                }
            };
            let Some(found_elem) = config
                .saved_positions
                .iter()
                .find(|position| position.name == position_name)
            else {
                eprintln!("Tray position `{position_name}` no longer exists");
                return;
            };
            let desk = match desk_mutex::get_desk_from_app_state(app) {
                Ok(desk) => desk,
                Err(error) => {
                    eprintln!("Could not move to tray position `{position_name}`: {error}");
                    return;
                }
            };
            block_on(async {
                if let Err(error) = loose_idasen::move_to_target(&desk, found_elem.value).await {
                    eprintln!("Failed to move desk to tray position `{position_name}`: {error}");
                }
            });
        }
    }
}

fn should_prevent_exit(code: Option<i32>) -> bool {
    code != Some(tauri::RESTART_EXIT_CODE)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        // Pass the desk instance to tauri to manage
        .manage(TauriSharedDesk(Mutex::new(Err(BtError::NotInitiated))))
        .setup(|app| {
            let app_handle = app.handle().clone();
            let startup = config_utils::startup_config(&app_handle);
            let config = startup.config;
            let recovery_error = startup.recovery_error;

            /*
            If there is a desk name present already, do not bother the end user with windows opening/loading. Just connect to his desk.
            */
            if recovery_error.is_none() {
                if let Some(local_name) = config.local_name.clone() {
                    let cached_desk =
                        block_on(loose_idasen::connect_to_desk_by_name_internal(local_name));
                    let initiated_desk = app.state::<TauriSharedDesk>();
                    desk_mutex::assign_desk_to_mutex(&initiated_desk, cached_desk);
                }
            }

            println!("Loaded config: {:?}", config);

            let tray_menu = config_utils::create_main_tray_menu(&app_handle, &config)?;
            let mut tray_builder = TrayIconBuilder::with_id(MAIN_TRAY_ID).menu(&tray_menu);
            if let Some(icon) = app.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            }
            #[cfg(target_os = "macos")]
            {
                tray_builder = tray_builder.icon_as_template(true);
            }
            tray_builder
                .on_menu_event(|app, event| handle_tray_menu_event(app, event.id().as_ref()))
                .build(app)?;

            // Preserve the initial config as managed state for existing setup behavior.
            app.manage(config.clone());

            if let Some(error) = recovery_error {
                eprintln!("Config recovery required: {error}");
                let init_script = config_recovery_initialization_script(&error);
                open_main_window(
                    &app_handle,
                    "Trayasen - Configuration recovery",
                    Some(&init_script),
                );
                return Ok(());
            }

            /*
                On setup, we only wanna bail early if we're already connected
                and register all the shortcuts
            */
            match &config.local_name {
                Some(actual_loc_name) => {
                    let desk_state = app.state::<TauriSharedDesk>();
                    match desk_state.0.lock() {
                        Ok(desk) => match desk.as_ref() {
                            Ok(desk) => {
                                for position in &config.saved_positions {
                                    register_position_shortcut(&app_handle, position, desk.clone());
                                }
                            }
                            Err(error) => {
                                let error = error.to_string();
                                let init_script = connection_error_initialization_script(
                                    actual_loc_name,
                                    error.as_str(),
                                );
                                open_main_window(
                                    &app_handle,
                                    "Trayasen - Woops!",
                                    Some(&init_script),
                                );
                                println!("opening error window! error: {error}");
                            }
                        },
                        Err(error) => eprintln!("Could not inspect connected desk state: {error}"),
                    };
                }
                None => open_main_window(&app_handle, "Trayasen - Setup", None),
            }

            Ok(())
        })
        // Pass functions invokable on frontend
        .invoke_handler(tauri::generate_handler![
            create_new_elem,
            config_utils::get_config,
            config_utils::remove_position,
            config_utils::remove_config,
            config_utils::reset_desk,
            loose_idasen::get_available_desks_to_connect,
            connect_to_desk_by_name,
            has_custom_decorations
        ])
        .enable_macos_default_menu(false)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |app_handle, event| match event {
            tauri::RunEvent::Ready => {}
            /*
                Exit requested, might mean that a new position has been added(or that just a window has been closed).
                This is troublesome; since all the positions are actually system tray elements, we need to re-instantiate the entire tray
                So, when we detected an exit requested, just to be safe, refresh the system tray.
                TODO: We should probably have a way of checking for new elements, to remove redundant system tray refreshes
            */
            tauri::RunEvent::ExitRequested { code, api, .. } => {
                println!("Exit requested with code {code:?}");
                if !should_prevent_exit(code) {
                    return;
                }

                match config_utils::get_config(app_handle.clone()).and_then(|config| {
                    config_utils::create_main_tray_menu(app_handle, &config)
                        .map_err(|error| error.to_string())
                }) {
                    Ok(main_menu) => match app_handle.tray_by_id(MAIN_TRAY_ID) {
                        Some(tray) => {
                            if let Err(error) = tray.set_menu(Some(main_menu)) {
                                eprintln!("Could not refresh the tray menu: {error}");
                            }
                        }
                        None => eprintln!("Could not refresh missing tray `{MAIN_TRAY_ID}`"),
                    },
                    Err(error) => eprintln!("Could not refresh tray config: {error}"),
                }

                // Closing a window keeps the background tray application alive.
                api.prevent_exit();
            }
            _ => {}
        });
}

#[cfg(test)]
mod initialization_script_tests {
    use super::*;
    use serde_derive::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct DecodedErrorWindowState {
        title: String,
        description: String,
        desk_name: String,
        error: String,
    }

    #[test]
    fn error_initialization_keeps_hostile_values_as_json_data() {
        let desk_name =
            "desk \"quoted\" \\\\ path\n</script><script>deskPayload()</script>\u{2028}";
        let error =
            "failure \"quoted\" \\\\ trace\n</script><script>errorPayload()</script>\u{2029}";
        let script = connection_error_initialization_script(desk_name, error);

        let state_json = script
            .strip_prefix("window.stateWorkaround = ")
            .and_then(|script| script.split_once(";\nhistory.replaceState({}, '', "))
            .map(|(state, _)| state)
            .expect("script should contain a JSON state assignment");
        let decoded: DecodedErrorWindowState = serde_json::from_str(state_json).unwrap();

        assert_eq!(
            decoded,
            DecodedErrorWindowState {
                title: format!(
                    "The app was not able to connect to your saved desk with name: `{desk_name}`."
                ),
                description: ERROR_DESCRIPTION.to_string(),
                desk_name: desk_name.to_string(),
                error: error.to_string(),
            }
        );
        assert!(!script.contains("</script>"));
        assert!(!script.contains('\u{2028}'));
        assert!(!script.contains('\u{2029}'));
        assert!(script.contains("\\n"));
        assert!(script.contains("\\\\"));
        assert!(script.contains("\\\""));
    }

    #[test]
    fn restart_exit_is_not_prevented() {
        assert!(!should_prevent_exit(Some(tauri::RESTART_EXIT_CODE)));
        assert!(should_prevent_exit(None));
        assert!(should_prevent_exit(Some(0)));
    }

    #[test]
    fn desk_name_is_persisted_only_after_successful_connection() {
        let mut persisted = false;
        let failed = persist_after_success::<()>(Err("connection failed".to_string()), || {
            persisted = true;
            Ok(())
        });
        assert!(failed.is_err());
        assert!(!persisted);

        let connected = persist_after_success(Ok("desk"), || {
            persisted = true;
            Ok(())
        });
        assert_eq!(connected.unwrap(), "desk");
        assert!(persisted);
    }

    #[test]
    fn route_initialization_keeps_hostile_route_as_json_data() {
        let route = "/error\"; routePayload(); //\\path\n</script>\u{2028}";
        let script = route_initialization_script(route);
        let route_json = script
            .strip_prefix("history.replaceState({}, '', ")
            .and_then(|script| script.strip_suffix(");"))
            .expect("script should contain a JSON route argument");

        assert_eq!(serde_json::from_str::<String>(route_json).unwrap(), route);
        assert!(!script.contains("</script>"));
        assert!(!script.contains('\u{2028}'));
    }
}
