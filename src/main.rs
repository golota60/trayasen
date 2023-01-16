use idasen::get_instance_by_mac;
use serde_derive::{Deserialize, Serialize};
use serde_json::from_str;
use std::str;
use std::{fs::read_to_string, process::Command};
use tao::menu::MenuId;
use tao::{
    event_loop::EventLoop,
    menu::{ContextMenu, MenuItemAttributes},
    system_tray::SystemTrayBuilder,
    window::Icon,
};

static CONFIG_FILE_NAME: &str = "config.json";
static FOLDER_NAME: &str = "idasen-tray";
static LINUX_DATA_DIR: &str = "$HOME/.local/share";

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Position {
    name: String,
    value: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ConfigData {
    saved_positions: Vec<Position>,
}

enum ConfigPaths {
    Linux,
    MacOS,
    Windows,
}

impl ConfigPaths {
    fn get_config_path(&self) -> String {
        let path = format!(
            "echo {}/{}/{}",
            LINUX_DATA_DIR, FOLDER_NAME, CONFIG_FILE_NAME
        );

        match self {
            ConfigPaths::Linux => {
                let home = Command::new("sh")
                    .arg("-c")
                    .arg(path)
                    .output()
                    .expect("Failed to get $HOME path");

                String::from_utf8(home.stdout).expect("Error while parsing $HOME path")
            }
            ConfigPaths::MacOS => todo!(),
            ConfigPaths::Windows => todo!(),
        }
    }
}

fn load_config() -> String {
    let config = ConfigPaths::Linux.get_config_path();
    let config = config.trim_end();
    config.to_string()
}

// EE:4D:A2:34:E4:8F
fn main() {
    let rt = tokio::runtime::Runtime::new().expect("Error while initializing runtime");
    let event_loop = EventLoop::new();

    let config_path = load_config();
    println!("Config path: {:?}", config_path);

    let config = {
        let file = read_to_string(config_path).expect("Error while reading config file");
        let config =
            from_str::<ConfigData>(file.as_str()).expect("Error while parsing config file");
        config
    };

    println!("Loaded config: {:?}", config);

    let desk = rt.block_on(async { get_instance_by_mac("EE:4D:A2:34:E4:8F").await });
    let desk = desk.expect("Error while connecting to the desk:");

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

    // TODO: have a nicer icon
    let icon = Icon::from_rgba(vec![70; 16], 2, 2).expect("error happen: ");

    let _system_tray = SystemTrayBuilder::new(icon, Some(main_tray))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _event_loop, _control_flow| match event {
        tao::event::Event::MenuEvent { menu_id, .. } => {
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
