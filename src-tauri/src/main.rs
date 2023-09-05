#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;
use btleplug::api::{Peripheral as ApiPeripheral};
use tauri_plugin_autostart::MacosLauncher;

use btleplug::platform::Peripheral as PlatformPeripheral;
use serde::Serialize;
use tauri::{async_runtime::block_on, Manager, SystemTray, SystemTrayEvent};
use tauri::GlobalShortcutManager;

mod config_utils;
mod loose_idasen;
mod tray_utils;

#[derive(Default)]
struct SharedDesk(Mutex<Option<PlatformPeripheral>>);

#[tauri::command]
fn create_new_elem(app_handle: tauri::AppHandle, name: &str, value: u16, shortcutvalue: Option<String>) -> String {
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
                shortcut: shortcutvalue.clone()
            });
            config_utils::update_config(&config);

            match shortcutvalue {
                Some(val) => {
                    shortcut.register(val.as_str(),  || {
                        println!("I should be moving");
                    });
                }
                _ => {}
            }

            "success".to_string()
        }
    }
}

enum SavedDeskStates {
    New,
    Saved,
}

impl SavedDeskStates {
    fn as_str(&self) -> &'static str {
        match self {
            SavedDeskStates::New => "new",
            SavedDeskStates::Saved => "saved",
        }
    }
}

#[derive(Serialize, Debug)]
struct PotentialDesk {
    name: String,
    status: String,
}

// https://github.com/tauri-apps/tauri/issues/2533 - this has to be a Result
/// Desk we're connecting to for UI info
#[tauri::command]
async fn get_desk_to_connect() -> Result<Vec<PotentialDesk>, ()> {
    let config = config_utils::get_or_create_config();
    let desk_list = loose_idasen::get_list_of_desks(&config.local_name).await;
    let desk_list_view = desk_list
        .iter()
        .map(|x| match config.local_name {
            Some(_) => PotentialDesk {
                name: x.name.to_string(),
                status: SavedDeskStates::Saved.as_str().to_string(),
            },
            None => PotentialDesk {
                name: x.name.to_string(),
                status: SavedDeskStates::New.as_str().to_string(),
            },
        })
        .collect::<Vec<PotentialDesk>>();

    println!("Connecting to desk: {:?}", &desk_list_view);

    Ok(desk_list_view)
}

async fn connect_to_desk_by_name_internal(
    name: String,
    desk: &SharedDesk,
) -> Result<PlatformPeripheral, ()> {
    let desk_to_connect = loose_idasen::get_list_of_desks(&Some(name.clone()))
        .await;
    let desk_to_connect = desk_to_connect.into_iter().next().expect("Error while getting a desk to connect to");
    let desk_to_connect = desk_to_connect.perp;
    println!("after desk to connect!");

    save_desk_name(&name).await;
    println!("saved desk!");
    loose_idasen::setup(&desk_to_connect).await;

    // println!("all set up!");
    // println!("assigned and connected!");

    Ok(desk_to_connect)
}

/// Provided a name, will connect to a desk with this name - after this step, desk actually becomes usable
#[tauri::command]
async fn connect_to_desk_by_name(
    name: String,
    desk: tauri::State<'_, SharedDesk>,
    app_handle: tauri::AppHandle
) -> Result<(), ()> {
    let config = config_utils::get_config();
    // let mut shortcut_manager = app_handle.global_shortcut_manager();
    let x = connect_to_desk_by_name_internal(name, &desk).await.unwrap();
    
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

async fn save_desk_name(name: &String) {
    let new_local_name = name;
    config_utils::save_local_name(new_local_name.clone());
}

fn main() {
    let config = config_utils::get_or_create_config();

    let desk = SharedDesk(None.into());

    let local_name = &config.local_name;
    block_on( async {
        if let Some(local_name) = local_name.clone() {
            let cached_desk = connect_to_desk_by_name_internal(local_name.clone(), &desk).await;
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
            if let Some(loc_name) = loc_name {
                app.get_window("main").unwrap().close();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_new_elem,
            config_utils::get_config,
            config_utils::remove_position,
            config_utils::remove_config,
            get_desk_to_connect,
            connect_to_desk_by_name,
        ])
        .enable_macos_default_menu(false)
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                config_utils::QUIT_ID => tray_utils::handle_exit_menu_click(),
                config_utils::ABOUT_ID => tray_utils::handle_about_menu_click(app),
                config_utils::ADD_POSITION_ID => tray_utils::handle_new_position_menu_click(app),
                config_utils::MANAGE_POSITIONS_ID => tray_utils::handle_manage_positions_menu_click(app),
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
                        let desk = app.state::<SharedDesk>();

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
