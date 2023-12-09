#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;
use tauri_plugin_autostart::MacosLauncher;

use btleplug::platform::Peripheral as PlatformPeripheral;
use tauri::GlobalShortcutManager;
use tauri::{async_runtime::block_on, Manager, SystemTray, SystemTrayEvent};

mod config_utils;
mod loose_idasen;
mod tray_utils;

#[derive(Default)]
pub struct TauriSharedDesk(Mutex<Option<PlatformPeripheral>>);

#[tauri::command]
fn create_new_elem(
    app_handle: tauri::AppHandle,
    name: &str,
    value: u16,
    shortcutvalue: Option<String>,
) -> String {
    let mut config = config_utils::get_config();
    let mut shortcut_manager = app_handle.global_shortcut_manager();

    println!("shortcut_acc: {:?}", shortcutvalue);

    let is_duplicate = config.saved_positions.iter().find(|elem| elem.name == name);
    match is_duplicate {
        Some(_) => {
            // Duplicate found
            "duplicate".to_string()
        }
        None => {
            // No duplicate
            config.saved_positions.push(config_utils::Position {
                name: name.to_string(),
                value,
                shortcut: shortcutvalue.clone(),
            });
            config_utils::update_config(&config);

            let desk = app_handle.state::<TauriSharedDesk>();
            let desk = desk.0.lock().expect("Error while unwrapping shared desk");
            let desk = desk
                .as_ref()
                .expect("Desk should have been defined at this point");

            let cloned_desk = desk.clone();
            if let Some(shortcut_acc) = shortcutvalue {
                if shortcut_acc != "" {
                    _ = shortcut_manager.register(shortcut_acc.as_str(), move || {
                        block_on(async {
                            loose_idasen::move_to_target(&cloned_desk, value)
                                .await
                                .unwrap();
                        });
                    });
                }
            }

            "success".to_string()
        }
    }
}

/// Provided a name, will connect to a desk with this name - after this step, desk actually becomes usable
#[tauri::command]
async fn connect_to_desk_by_name(name: String) -> Result<(), ()> {
    _ = loose_idasen::connect_to_desk_by_name_internal(name).await;

    Ok(())
}

fn main() {
    let config = config_utils::get_or_create_config();
    let initiated_desk = TauriSharedDesk(None.into());

    /*
    If there is a desk name present already, do not bother the end user with windows opening/loading. Just connect to his desk.
    */
    let local_name = &config.local_name;
    block_on(async {
        if let Some(local_name) = local_name.clone() {
            let cached_desk = loose_idasen::connect_to_desk_by_name_internal(local_name.clone())
                .await
                .ok();

            *initiated_desk
                .0
                .lock()
                .expect("Failed to deref mutex during instantiation") = cached_desk;
        }
    });

    println!("Loaded config: {:?}", config);

    let tray_skeleton = config_utils::create_main_tray_menu(&config);
    let tray = SystemTray::new().with_menu(tray_skeleton);

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        // Pass the tray instance to tauri to manage
        .system_tray(tray)
        // Pass the desk instance to tauri to manage
        .manage(initiated_desk)
        // Pass the previously instantiates config. We ideally want to read fs only once.
        .manage(config)
        .setup(|app| {
            /*
                On setup, we only wanna bail early if we're already connected
                and register all the shortcuts
            */
            let config = app.state::<config_utils::ConfigData>();

            let loc_name = &config.local_name;
            let window = app.get_window("main").unwrap();

            if let Some(_) = loc_name {
                let desk_state = app.state::<TauriSharedDesk>();

                // We expect the desk to already exist at this point, since if loc_name, the first thing we do in the app is connect
                let desk = desk_state
                    .0
                    .lock()
                    .expect("Error while unwrapping shared desk");
                let desk = desk.as_ref();
                match desk {
                    /*
                        If the user is returning(has a config) immidiately close the window, not to eat resources
                        And then proceed to try to create the menu.
                    */
                    Some(desk) => {
                        window
                            .close()
                            .expect("Error while closing the initial window");
                        // Register all shortcuts
                        let mut shortcut_manager = app.global_shortcut_manager();
                        let all_positions = &config.saved_positions;
                        let cloned_pos = all_positions.clone();
                        for pos in cloned_pos.into_iter() {
                            // Each iteration needs it's own clone; we do not want to consume the app state
                            let cloned_desk = desk.clone();
                            if let Some(shortcut_key) = &pos.shortcut {
                                if shortcut_key != "" {
                                    _ = shortcut_manager.register(
                                        shortcut_key.as_str(),
                                        move || {
                                            block_on(async {
                                                loose_idasen::move_to_target(
                                                    &cloned_desk,
                                                    pos.value,
                                                )
                                                .await
                                                .unwrap();
                                            });
                                        },
                                    );
                                }
                            }
                        }
                    }
                    None => {
                        // Open error window with the error
                        println!("opening error window!");
                        _ = window.set_title("Trayasen - Woops!");
                        window
                            .show()
                            .expect("Error while trying to show the window");
                        _ = window.eval(
                            r#"
                        history.replaceState({}, '','/error');
                        "#,
                        );
                    }
                }
            } else {
                window
                    .show()
                    .expect("Error while trying to show the window");
            };

            Ok(())
        })
        // Pass functions invokable on frontend
        .invoke_handler(tauri::generate_handler![
            create_new_elem,
            config_utils::get_config,
            config_utils::remove_position,
            config_utils::remove_config,
            loose_idasen::get_desk_to_connect,
            connect_to_desk_by_name,
        ])
        .enable_macos_default_menu(false)
        // Register all the tray events, eg. clicks and stuff
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                config_utils::QUIT_ID => tray_utils::handle_exit_menu_click(),
                config_utils::ABOUT_ID => tray_utils::handle_about_menu_click(app),
                config_utils::ADD_POSITION_ID => tray_utils::handle_new_position_menu_click(app),
                config_utils::MANAGE_POSITIONS_ID => {
                    tray_utils::handle_manage_positions_menu_click(app)
                }
                // If event is not one of predefined, assume a position has been clicked
                remaining_id => {
                    // Get config one more time, in case there's a new position added since intialization
                    let config = config_utils::get_config();
                    let updated_menus = config_utils::get_menu_items_from_config(&config);
                    let found_elem = updated_menus
                        .iter()
                        .find(|pos| pos.position_elem.id_str == remaining_id)
                        .expect("Clicked element not found");
                    block_on(async {
                        let desk = app.state::<TauriSharedDesk>();

                        let desk = desk.0.lock();
                        let desk = desk.expect("Error while unwrapping shared desk");
                        let desk = desk
                            .as_ref()
                            .expect("Desk should have been defined at this point");

                        loose_idasen::move_to_target(desk, found_elem.value)
                            .await
                            .unwrap();
                    });
                }
            },
            _ => {}
        })
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
            tauri::RunEvent::ExitRequested { api, .. } => {
                println!("Exit requested");
                let config = config_utils::get_config();
                let main_menu = config_utils::create_main_tray_menu(&config);
                app_handle
                    .tray_handle()
                    .set_menu(main_menu)
                    .expect("Error whilst unwrapping main menu");

                // Do not actually exit the app
                api.prevent_exit();
            }
            _ => {}
        });
}
