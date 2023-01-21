use tao::menu::MenuId;
use tao::{
    event::Event,
    event_loop::EventLoop,
    menu::{ContextMenu, MenuItemAttributes},
    system_tray::SystemTrayBuilder,
};

mod config_utils;
mod local_idasen;

fn main() {
    let rt = tokio::runtime::Runtime::new().expect("Error while initializing runtime");
    let event_loop = EventLoop::new();

    let config = config_utils::load_config();

    println!("Loaded config: {:?}", config);

    let mac_address = config.mac_address;

    let desk = rt
        .block_on(local_idasen::get_universal_instance(&mac_address))
        .expect("Error while unwrapping local idasen instance");

    // Save the desk's MAC address, if not present
    if mac_address.is_none() {
        let new_mac_address = desk.mac_addr;
        config_utils::save_mac_address(new_mac_address);
    }

    let mut main_tray = ContextMenu::new();
    let mut conf_list_submenu = ContextMenu::new();

    // Header - does nothing, this is more of a decorator
    let tray_header = MenuItemAttributes::new("Idasen controller").with_enabled(false);
    main_tray.add_item(tray_header);

    let menu_ids = config
        .saved_positions
        .iter()
        .map(|temp_conf_elem| {
            // Assign values so that they are not lost - TODO: figure out why the fuck does that even happen
            let name = &temp_conf_elem.name;
            let value = &temp_conf_elem.value;
            let conf_item_title = name.as_str().clone();
            let conf_item_menuid = MenuId::new(conf_item_title);
            let conf_item = MenuItemAttributes::new(conf_item_title).with_id(conf_item_menuid);
            conf_list_submenu.add_item(conf_item);
            (conf_item_menuid.clone(), name.clone(), value.clone())
        })
        .collect::<Vec<(MenuId, String, u16)>>();

    main_tray.add_submenu("Profiles", true, conf_list_submenu);

    let tray_quit_id = MenuId::new("Quit");
    let tray_quit = MenuItemAttributes::new("Quit").with_id(tray_quit_id);
    main_tray.add_item(tray_quit);

    let icon = config_utils::load_icon(std::path::Path::new("./assets/carrot_1.png"));

    let _system_tray = SystemTrayBuilder::new(icon, Some(main_tray))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _event_loop, _control_flow| match event {
        Event::MenuEvent { menu_id, .. } => {
            if menu_id == tray_quit_id {
                std::process::exit(0);
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
