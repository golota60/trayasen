mod desk_mutex;
mod config_utils;
mod loose_idasen;
mod tray_utils;
mod tests;

use std::sync::Mutex;
use tauri::{Manager, WebviewWindowBuilder, WebviewUrl, tray::TrayIconBuilder};
use tauri_plugin_global_shortcut::GlobalShortcutExt;
use futures::executor::block_on;
// use window_shadows::set_shadow;
use btleplug::platform::Peripheral as PlatformPeripheral;
use crate::loose_idasen::BtError;

pub struct TauriSharedDesk(Mutex<Result<PlatformPeripheral, BtError>>);

// Whether a system should have custom decorations or not
#[tauri::command]
fn has_custom_decorations() -> bool {
    if cfg!(windows) {
        return true;
    }
    false
}

pub trait WindowInitUtils<R: tauri::Runtime> {
    fn init_trayasen(self, title: &str, err_msg: &str, init_script: Option<&str>) -> tauri::Result<tauri::WebviewWindow<R>>;
}

impl<R: tauri::Runtime, M: tauri::Manager<R>> WindowInitUtils<R> for WebviewWindowBuilder<'_, R, M> {
    fn init_trayasen(self, title: &str, err_msg: &str, init_script: Option<&str>) -> tauri::Result<tauri::WebviewWindow<R>> {
        // We want to replace borders only on windows, as on macOS they are pretty enough, and on Linux it's not supported by `window_shadows`
        let mut window_builder = if has_custom_decorations() {
            self.inner_size(1280.0, 720.0).title(title).always_on_top(true).decorations(false)
        } else {
            self.inner_size(1280.0, 720.0).title(title).always_on_top(true)
        };

        if let Some(init_script) = init_script {
            window_builder = window_builder.initialization_script(init_script);
        }

        let window_instance= window_builder.build().expect(err_msg);
        // TODO: Re-enable shadows when window-shadows supports Tauri v2
        // if has_custom_decorations() {
        //     set_shadow(&window_instance, true).unwrap();
        // }
        Ok(window_instance)
    }
}


#[tauri::command]
fn create_new_elem(
    app_handle: tauri::AppHandle,
    name: &str,
    value: u16,
    shortcutvalue: Option<String>,
) -> String {
    let mut config = config_utils::get_config(app_handle.clone());
    let shortcut_manager = app_handle.global_shortcut();

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
            config_utils::update_config(&app_handle, &config);

            let desk = desk_mutex::get_desk_from_app_state(&app_handle);

            let _cloned_desk = desk.clone();
            if let Some(shortcut_acc) = shortcutvalue {
                if shortcut_acc != "" {
                    _ = shortcut_manager.register(shortcut_acc.as_str());
                }
            }

            "success".to_string()
        }
    }
}

/// Provided a name, will connect to a desk with this name - after this step, desk actually becomes usable
#[tauri::command]
async fn connect_to_desk_by_name(app_handle: tauri::AppHandle, name: String) -> Result<(), String> {
    println!("connecting to desk with name: {}", name);
    let instantiated_desk = app_handle.state::<TauriSharedDesk>();
    println!("with desk!...");
    let cached_desk = loose_idasen::connect_to_desk_by_name_internal(name).await;
    println!("after cached desk...");
    if cached_desk.is_err() {
        println!("in error!...");
        return Err(cached_desk.unwrap_err().to_string());
    }

    println!("cached desk: some:{}, none:{}", cached_desk.is_ok(), cached_desk.is_err());
    desk_mutex::assign_desk_to_mutex(&instantiated_desk, cached_desk);
    println!("Successfuly connected to desk from frontend");
    Ok(())
}


fn load_icon() -> tauri::image::Image<'static> {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(include_bytes!("../icons/carrot.png"))
            .expect("Failed to load icon")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tauri::image::Image::new_owned(icon_rgba, icon_width, icon_height)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let initiated_desk = TauriSharedDesk(Mutex::new(Err(BtError::NotInitiated)));
            let config = config_utils::get_or_create_config(app.app_handle());
            let local_name = &config.local_name;
            block_on(async {
                if let Some(local_name) = local_name.clone() {
                    let cached_desk = loose_idasen::connect_to_desk_by_name_internal(local_name.clone())
                        .await;

                    desk_mutex::assign_desk_to_mutex(&initiated_desk, cached_desk);
                }
            });

            let tray_skeleton = config_utils::create_main_tray_menu(app.app_handle(), &config)?;
            let tray_builder = TrayIconBuilder::new()
                .icon(load_icon())
                .menu(&tray_skeleton);

            let _ = tray_builder.build(app)?;
            
            app.manage(initiated_desk);
            app.manage(config);
            
            let app_handle = app.handle().clone();
            app.on_menu_event(move |app, event| {
                match event.id().as_ref() {
                    config_utils::QUIT_ID => tray_utils::handle_exit_menu_click(),
                    config_utils::ABOUT_ID => tray_utils::handle_about_menu_click(&app_handle),
                    config_utils::ADD_POSITION_ID => tray_utils::handle_new_position_menu_click(&app_handle),
                    config_utils::MANAGE_POSITIONS_ID => tray_utils::handle_manage_positions_menu_click(&app_handle),
                    remaining_id => {
                        let config = config_utils::get_config(app.app_handle().clone());
                        let updated_menus = config_utils::get_menu_items_from_config(app.app_handle(), &config).unwrap();
                        let found_elem = updated_menus
                            .iter()
                            .find(|pos| pos.position_elem.id().as_ref() == remaining_id);
                        
                        if let Some(found_elem) = found_elem {
                            block_on(async {
                                let desk = desk_mutex::get_desk_from_app_state(app.app_handle());
                                loose_idasen::move_to_target(&desk, found_elem.value).await.unwrap();
                            });
                        }
                    }
                }
            });

            let config = app.state::<config_utils::ConfigData>();
            let loc_name = &config.local_name;

            match loc_name {
                Some(actual_loc_name) => {
                    let desk_state = app.state::<TauriSharedDesk>();

                    let desk = desk_state
                        .0
                        .lock()
                        .expect("Error while unwrapping shared desk");
                    let desk = desk.as_ref();
                    match desk {
                        Ok(desk) => {
                            let all_positions = &config.saved_positions;
                            let cloned_pos = all_positions.clone();

                            for pos in cloned_pos.into_iter() {
                                let _cloned_desk = desk.clone();
                                if let Some(shortcut_key) = &pos.shortcut {
                                    if shortcut_key != "" {
                                        let _ = app.global_shortcut().register(shortcut_key.as_str());
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            let err_window = WebviewWindowBuilder::new(app, "init_window", WebviewUrl::App("index.html".into())).init_trayasen("Trayasen - Woops!","Error while creating window", None);
                            
                            println!("opening error window! error: {}", e);
                            
                            _ = err_window?.eval(&format!(
                                r#"
                                window.stateWorkaround = {{
                                    title: "The app was not able to connect to your saved desk with name: `{}`.",
                                    description: "Either try reconnecting with that desk from your system and relaunch Trayasen, or click the button below to run the setup again.",
                                    desk_name: "{}",
                                    error: "{}"
                                }}
                                history.replaceState({{}}, '','/error');
                        "#,
                                actual_loc_name,
                                actual_loc_name,
                                e.to_string(),
                            ))?;
                        }
                    }
                }
                None => {
                    let init_window = WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into())).init_trayasen("Trayasen - Setup", "Error while creating window", None);
                    
                    init_window?                        .show()
                        .expect("Error while trying to show the window");

                    
                    // TODO: Re-enable shadows when window-shadows supports Tauri v2
                    // #[cfg(any(windows, target_os = "macos"))]
                    // set_shadow(&init_window?, true).unwrap();
                }
            }

            Ok(())
        })
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
        .run(tauri::generate_context!()).expect("error while running tauri application");
}
