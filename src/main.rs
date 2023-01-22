use std::cell::RefCell;
use std::rc::Rc;

use gtk::Window;
use gtk::{prelude::*, Orientation};
use idasen::{MAX_HEIGHT, MIN_HEIGHT};
use tao::{event::Event, event_loop::EventLoop, system_tray::SystemTrayBuilder};

mod config_utils;
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

    let system_tray = SystemTrayBuilder::new(icon, Some(menu))
        .build(&event_loop)
        .unwrap();

    // No idea what is this shit - https://users.rust-lang.org/t/cannot-borrow-calc-as-mutable-as-it-is-a-captured-variable-in-a-fn-closure/40042
    let cloned_tray = Rc::new(RefCell::new(system_tray));
    event_loop.run(move |event, _event_loop, _control_flow| match event {
        Event::MenuEvent { menu_id, .. } => {
            if menu_id == tray_quit_id {
                std::process::exit(0);
            } else if menu_id == tray_new_id {
                let name_info = gtk::Label::new(Some("Position name"));
                let height_info =
                    format!("Height: a number between {} and {}", MIN_HEIGHT, MAX_HEIGHT);
                let height_info = gtk::Label::new(Some(&height_info));
                let content_wrapper = gtk::Box::builder()
                    .orientation(Orientation::Vertical)
                    .margin(12)
                    .build();
                let button = gtk::Button::builder().label("Add a position").build();

                let name_input = gtk::Entry::new();
                let value_input = gtk::Entry::new();
                content_wrapper.pack_start(&name_info, true, true, 10);
                content_wrapper.pack_start(&name_input, true, true, 10);
                content_wrapper.pack_start(&height_info, true, true, 10);
                content_wrapper.pack_start(&value_input, true, true, 10);
                content_wrapper.pack_start(&button, true, true, 10);

                let gtk_window = Window::builder().resizable(false).build();

                gtk_window.set_size_request(256, 128);
                gtk_window.add(&content_wrapper);
                gtk_window.show_all();

                let lol = Rc::clone(&cloned_tray);
                button.connect_clicked(move |_| {
                    let new_name = name_input.text().to_string();
                    let new_value = value_input
                        .text()
                        .to_string()
                        .parse::<u16>()
                        .expect("Value should be a number");

                    // TODO: validate input better than exiting app
                    let mut config = config_utils::get_config();
                    config.saved_positions.push(config_utils::Position {
                        name: new_name,
                        value: new_value,
                    });
                    config_utils::update_config(&config);

                    let new_menu = config_utils::create_main_tray(&config).menu;
                    // Followup to the shit i did upwards
                    let mut borrowed_cloned_tray = lol.borrow_mut();
                    borrowed_cloned_tray.set_menu(&new_menu);

                    gtk_window.close();
                });
            } else {
                let found_elem = menu_ids
                    .iter()
                    .find(|pos| pos.0 == menu_id)
                    .expect("Clicked element not found");
                rt.block_on(async {
                    println!("Moving the table");
                    let target_height = found_elem.2;
                    desk.move_to(target_height).await
                })
                .unwrap();
            }
        }
        _ => {}
    });
}
