#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu};

mod config_utils;
mod local_idasen;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let rt = tokio::runtime::Runtime::new().expect("Error while initializing runtime");
    // let event_loop = EventLoop::new();

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
        quit_item,
        add_position_item,
    } = config_utils::create_main_tray(&config);

    tauri::Builder::default()
        .system_tray(tray)
        .invoke_handler(tauri::generate_handler![greet])
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
