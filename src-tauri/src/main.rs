#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::SystemTrayEvent;

mod config_utils;
mod local_idasen;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let rt = tokio::runtime::Runtime::new().expect("Error while initializing runtime");

    let config = config_utils::load_config();

    println!("Loaded config: {:?}", config);

    let mac_address = &config.mac_address;

    let desk = rt
        .block_on(local_idasen::get_universal_instance(&mac_address))
        .expect("Error while unwrapping local idasen instance");

    // Save the desk's MAC address, if not present
    if mac_address.is_none() {
        let new_mac_address = desk.mac_addr;
        config_utils::save_mac_address(new_mac_address);
    }

    let config_utils::MainTrayData {
        tray,
        position_menu_items,
    } = config_utils::create_main_tray(&config);

    tauri::Builder::default()
        .system_tray(tray)
        .invoke_handler(tauri::generate_handler![greet])
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                config_utils::QUIT_ID => {
                    std::process::exit(0);
                }
                config_utils::ABOUT_ID => {
                    tauri::WindowBuilder::new(
                        app,
                        "main",
                        tauri::WindowUrl::App("index.html".into()),
                    )
                    .always_on_top(true)
                    .build()
                    .expect("Error while trying to open about window");
                }
                config_utils::ADD_POSITION_ID => {
                    tauri::WindowBuilder::new(
                        app,
                        "main",
                        tauri::WindowUrl::App("index.html".into()),
                    )
                    .always_on_top(true)
                    .build()
                    .expect("Error while trying to open new postition window");
                }
                remaining_id => {
                    // Check whether a position has been clicked
                    // Get config one more time, in case there's a new position added since intialization
                    let config = config_utils::get_config();
                    let updated_menus = config_utils::get_menu_items_from_config(&config);
                    let found_elem = updated_menus
                        .iter()
                        .find(|pos| pos.position_elem.id_str == remaining_id)
                        .expect("Clicked element not found");
                    rt.block_on(async {
                        println!(
                            "Moving the table. Pos name: {}. Pos height:{}",
                            found_elem.name, found_elem.value
                        );
                        let target_height = found_elem.value;
                        desk.move_to(target_height).await
                    })
                    .unwrap();
                }
            },
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });

    println!("after create");
}
