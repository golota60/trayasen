#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{sync::Mutex, thread};
use tauri_plugin_autostart::MacosLauncher;

use btleplug::platform::Peripheral as PlatformPeripheral;
use serde::Serialize;
use tauri::{async_runtime::block_on, Manager, SystemTray, SystemTrayEvent};

mod config_utils;
mod loose_idasen;

#[derive(Default)]
struct SharedDesk(Mutex<Option<PlatformPeripheral>>);

#[tauri::command]
fn create_new_elem(name: &str, value: u16) -> String {
    let mut config = config_utils::get_config();

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
            });
            config_utils::update_config(&config);

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
    desk: tauri::State<'_, SharedDesk>,
) -> Result<(), ()> {
    let desk_to_connect = loose_idasen::get_list_of_desks(&Some(name.clone()))
        .await
        .first()
        .expect("Error while getting a desk to connect to")
        .perp
        .clone();
    println!("after desk to connect!");

    save_desk_name(&name).await;
    println!("saved desk!");
    loose_idasen::setup(&desk_to_connect).await;

    println!("all set up!");
    *desk.0.lock().unwrap() = Some(desk_to_connect);
    println!("assigned and connected!");

    Ok(())
}

/// Provided a name, will connect to a desk with this name - after this step, desk actually becomes usable
#[tauri::command]
async fn connect_to_desk_by_name(
    name: String,
    desk: tauri::State<'_, SharedDesk>,
) -> Result<(), ()> {
    connect_to_desk_by_name_internal(name, desk).await
}

async fn save_desk_name(name: &String) {
    let new_local_name = name;
    config_utils::save_local_name(new_local_name.clone());
}

fn main() {
    let config = config_utils::get_or_create_config();

    println!("Loaded config: {:?}", config);

    let local_name = &config.local_name;

    let tray = config_utils::create_main_tray_menu(&config);
    let tray = SystemTray::new().with_menu(tray);

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .system_tray(tray)
        .manage(SharedDesk(None.into()))
        .setup(move |app| {
            let loc_name = &config.local_name;
            match loc_name {
                // If saved name is defined, don't open the initial window
                Some(e) => {
                    let win = app
                        .get_window("main")
                        .unwrap();
                    // We need to connect to the desk from a javascript level(unfortunately)
                    win.show();

                    // println!(
                    //     "config found. closing main window. name: {:?}",
                    //     e.to_string()
                    // );

                    // block_on(async {
                    //     connect_to_desk_by_name_internal(e.to_string(), app.state::<SharedDesk>())
                    //         .await;
                    // });
                    // println!("after connect by name");

                }
                None => {
                    let win = app
                        .get_window("main")
                        .expect("Error while getting main window window on init");
                    win.show();
                }
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
                config_utils::QUIT_ID => {
                    std::process::exit(0);
                }
                config_utils::NOTIFY_CONNECT_ID => {
                    match tauri::WindowBuilder::new(
                        app,
                        config_utils::NOTIFY_CONNECT_ID,
                        tauri::WindowUrl::App("index.html".into()),
                    )
                    .always_on_top(true)
                    .initialization_script(
                        r#"
                    history.replaceState({}, '','/autoconnect');
                    "#,
                    )
                    .title("Trayasen - About/Options")
                    .build()
                    {
                        Ok(_) => {}
                        Err(_) => {
                            println!("Error while trying to open about window");
                        }
                    }
                }
                config_utils::ABOUT_ID => {
                    match tauri::WindowBuilder::new(
                        app,
                        "main",
                        tauri::WindowUrl::App("index.html".into()),
                    )
                    .always_on_top(true)
                    .initialization_script(
                        r#"
                    history.replaceState({}, '','/about');
                    "#,
                    )
                    .title("Trayasen - About/Options")
                    .build()
                    {
                        Ok(_) => {}
                        Err(_) => {
                            println!("Error while trying to open about window");
                        }
                    }
                }
                config_utils::ADD_POSITION_ID => {
                    match tauri::WindowBuilder::new(
                        app,
                        "main",
                        tauri::WindowUrl::App("index.html".into()),
                    )
                    .always_on_top(true)
                    .initialization_script(
                        r#"
                    history.replaceState({}, '','/new-position');
                    "#,
                    )
                    .title("Trayasen - Add position")
                    .build()
                    {
                        Ok(_) => {}
                        Err(_) => {
                            println!("Error while trying to open new postition window");
                        }
                    }
                }
                config_utils::MANAGE_POSITIONS_ID => {
                    match tauri::WindowBuilder::new(
                        app,
                        "main",
                        tauri::WindowUrl::App("index.html".into()),
                    )
                    .always_on_top(true)
                    .initialization_script(
                        r#"
                    history.replaceState({}, '','/manage-positions');
                    "#,
                    )
                    .title("Trayasen - Manage positions")
                    .build()
                    {
                        Ok(_) => {}
                        Err(_) => {
                            println!("Error while trying to open manage positions window");
                        }
                    }
                }
                // Means a position name has been clicked
                remaining_id => {
                    // Check whether a position has been clicked
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
                        let state = app.state::<SharedDesk>();

                        let desk = state;
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
        .run(move |_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                // Exit requested might mean that a new element has been added.
                let config = config_utils::get_config();
                let main_menu = config_utils::create_main_tray_menu(&config);
                _app_handle
                    .tray_handle()
                    .set_menu(main_menu)
                    .expect("Error whilst unwrapping main menu");

                api.prevent_exit();
            }
            _ => {}
        });

    println!("after create");
}
