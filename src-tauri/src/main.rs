#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, SystemTray, SystemTrayEvent};

mod broken_idasen;
mod config_utils;
mod local_idasen;

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

fn main() {
    let rt = tokio::runtime::Runtime::new().expect("Error while initializing runtime");

    let config = config_utils::get_or_create_config();

    println!("Loaded config: {:?}", config);

    let local_name = &config.local_name;

    let power_desk = rt
        .block_on(local_idasen::get_universal_instance(&local_name))
        .expect("Error while unwrapping local idasen instance");
    let desk = power_desk.actual_idasen;

    // Save the desk's MAC address, if not present
    if local_name.is_none() {
        let new_local_name = power_desk.local_name;
        // println!("{:?}", desk);
        config_utils::save_local_name(new_local_name);
    }

    let tray = config_utils::create_main_tray_menu(&config);
    let tray = SystemTray::new().with_menu(tray);

    tauri::Builder::default()
        .system_tray(tray)
        .invoke_handler(tauri::generate_handler![
            create_new_elem,
            config_utils::get_config,
            config_utils::remove_position,
            // local_idasen::get_test
        ])
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
                    .initialization_script(
                        r#"
                    history.replaceState({}, '','/about');
                    "#,
                    )
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
                    .initialization_script(
                        r#"
                    history.replaceState({}, '','/new-position');
                    "#,
                    )
                    .build()
                    .expect("Error while trying to open new postition window");
                }
                config_utils::MANAGE_POSITIONS_ID => {
                    tauri::WindowBuilder::new(
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
                    .build()
                    .expect("Error while trying to open manage positions window");
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
        .run(move |_app_handle, event| match event {
            tauri::RunEvent::Ready => {
                // Immidiately close the window if user has done the initialization
                let is_init_done = config.saved_positions.len() > 0;

                if is_init_done {
                    let win = _app_handle
                        .get_window("main")
                        .expect("Error while getting main window window on init");
                    win.close().expect("Error while closing the window");
                }
            }
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
