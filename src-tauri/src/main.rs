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
    let mut shortcut = app_handle.global_shortcut_manager();

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

            match shortcutvalue {
                Some(val) => {
                    shortcut.register(val.as_str(), || {
                        println!("I should be moving");
                    });
                }
                _ => {}
            }

            "success".to_string()
        }
    }
}

/// Provided a name, will connect to a desk with this name - after this step, desk actually becomes usable
#[tauri::command]
async fn connect_to_desk_by_name(
    name: String,
    desk: tauri::State<'_, TauriSharedDesk>,
    app_handle: tauri::AppHandle,
) -> Result<(), ()> {
    let config = config_utils::get_config();
    // let mut shortcut_manager = app_handle.global_shortcut_manager();
    let x = loose_idasen::connect_to_desk_by_name_internal(name, &desk)
        .await
        .unwrap();

    // *desk.0.lock().unwrap() = Some(x);

    // // After connecting, register shortcuts
    //                 // TODO: REPORT ALL SHORTCUTS HERE
    //                 let positions = config.saved_positions;
    //                 for pos in positions.iter() {
    //                     let shortcut_value = pos.shortcut.clone();
    //                     if let Some(shortcut_value) = shortcut_value {
    //                         shortcut_manager.register(shortcut_value.as_str(), || {
    //                             connect_to_desk_by_name_internal(config.local_name.clone().unwrap().to_string(), desk);
    //                         });
    //                     }
    //                 };
    //                 x

    Ok(())
}

fn main() {
    let config = config_utils::get_or_create_config();

    let desk = TauriSharedDesk(None.into());

    // Get the desk before running the app if possible, so that user doesn't see any loading screens
    // instead app just starts up slower
    let local_name = &config.local_name;
    block_on(async {
        if let Some(local_name) = local_name.clone() {
            let cached_desk =
                loose_idasen::connect_to_desk_by_name_internal(local_name.clone(), &desk).await;
            *desk.0.lock().unwrap() = Some(cached_desk.unwrap());
        }
    });

    println!("Loaded config: {:?}", config);

    let tray = config_utils::create_main_tray_menu(&config);
    let tray = SystemTray::new().with_menu(tray);

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .system_tray(tray)
        .manage(desk)
        .setup(move |app| {
            let loc_name = &config.local_name;
            let window = app.get_window("main").unwrap();
            if let Some(loc_name) = loc_name {
                window.close();
                let mut shortcut_manager = app.global_shortcut_manager();
                let all_positions = &config.saved_positions;
                all_positions.iter().for_each(|pos| {
                    if let Some(shortcut_key) = &pos.shortcut {
                        shortcut_manager.register(shortcut_key.as_str(), || {
                            println!("I should be moving");
                        });
                    }
                });
            } else {
                window.show();
            };

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_new_elem,
            config_utils::get_config,
            config_utils::remove_position,
            config_utils::remove_config,
            loose_idasen::get_desk_to_connect,
            connect_to_desk_by_name,
        ])
        .enable_macos_default_menu(false)
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
                    println!("something has been clicked");
                    let config = config_utils::get_config();
                    let updated_menus = config_utils::get_menu_items_from_config(&config);
                    let found_elem = updated_menus
                        .iter()
                        .find(|pos| pos.position_elem.id_str == remaining_id)
                        .expect("Clicked element not found");
                    block_on(async {
                        let target_height = found_elem.value;
                        let desk = app.state::<TauriSharedDesk>();

                        let desk = desk;
                        let desk = desk.0.lock();
                        let desk = desk.expect("Error while unwrapping shared desk");
                        let desk = desk
                            .as_ref()
                            .expect("Desk should have been defined at this point");

                        loose_idasen::move_to_target(desk, found_elem.value).await;
                    })
                }
            },
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |app_handle, event| match event {
            tauri::RunEvent::Ready => {
                let config = config_utils::get_config();
                let mut shortcut_manager = app_handle.global_shortcut_manager();
                // let desk = app_handle.state::<SharedDesk>();

                // TODO: HANDLE DUPLICATED SHORTCUTS
            }
            tauri::RunEvent::ExitRequested { api, .. } => {
                // Exit requested might mean that a new element has been added.
                let config = config_utils::get_config();
                let main_menu = config_utils::create_main_tray_menu(&config);
                app_handle
                    .tray_handle()
                    .set_menu(main_menu)
                    .expect("Error whilst unwrapping main menu");

                api.prevent_exit();
            }
            _ => {}
        });
}
