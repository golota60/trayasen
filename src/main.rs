use eframe::egui;
use tao::{event::Event, event_loop::EventLoop, system_tray::SystemTrayBuilder};

mod config_utils;
mod egui_utils;
mod local_idasen;

fn main() {
    let rt = tokio::runtime::Runtime::new().expect("Error while initializing runtime");
    let event_loop = EventLoop::new();

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
        menu,
        menu_ids,
        tray_quit_id,
        tray_new_id,
    } = config_utils::create_main_tray(&config);

    let icon = config_utils::load_icon(std::path::Path::new("./assets/carrot_1.png"));

    let mut system_tray = SystemTrayBuilder::new(icon, Some(menu))
        .build(&event_loop)
        .unwrap();
    event_loop.run(move |event, _event_loop, _control_flow| match event {
        Event::MenuEvent { menu_id, .. } => {
            if menu_id == tray_quit_id {
                std::process::exit(0);
            } else if menu_id == tray_new_id {
                // Initializing a whole ass egui app on each opened window? sounds extemely idiotic, but fuck it
                let options = eframe::NativeOptions {
                    initial_window_size: Some(egui::vec2(320.0, 240.0)),
                    run_and_return: true,
                    resizable: false,
                    centered: true,
                    ..Default::default()
                };
                eframe::run_native(
                    "Add new position",
                    options,
                    Box::new(move |_cc| Box::new(egui_utils::MyApp::default())),
                );

                let config = config_utils::get_config();
                let new_menu = config_utils::create_main_tray(&config).menu;
                system_tray.set_menu(&new_menu);
            } else {
                // Get config one more time, in case there's a new position added since intialization
                let config = config_utils::get_config();
                let updated_menus = config_utils::get_menus_from_config(&config);
                let found_elem = updated_menus
                    .iter()
                    .find(|pos| pos.elem_menu_id == menu_id)
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
        }
        _ => {}
    });
}
