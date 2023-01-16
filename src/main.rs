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

/**
 * TODO list:
 * 1. store data somewhere in the system(system specific)
 * 2. save & load stuff to that file(json probably)
 */

#[derive(Deserialize, Serialize, Debug)]
struct Position {
    name: String,
    value: u16,
}

#[derive(Deserialize, Serialize, Debug)]
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
                let mut home = Command::new("sh")
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
    let sys = cfg!(windows);
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

        from_str::<ConfigData>(&file).expect("Error while parsing config file")
    };

    println!("Loaded config: {:?}", config);

    let desk = rt.block_on(async { get_instance_by_mac("EE:4D:A2:34:E4:8F").await });
    let desk = desk.expect("Error while connecting to the desk:");

    let mut main_tray = ContextMenu::new();
    let mut conf_list_submenu = ContextMenu::new();

    // Header - does nothing, this is more of a decorator - maybe there's a better way than disable button?
    let tray_header = MenuItemAttributes::new("Idasen controller").with_enabled(false);
    main_tray.add_item(tray_header);

    // TODO: Have an exit button
    // TODO: Spawn item for each config elem and assign click actions to them
    let configs = config.saved_positions;
    let mut menu_ids = configs
        .iter()
        .map(|temp_conf_elem| {
            let conf_item_title = temp_conf_elem.name.as_str();
            let conf_item_menuid = MenuId::new(conf_item_title);
            let conf_item = MenuItemAttributes::new(conf_item_title).with_id(conf_item_menuid);
            conf_list_submenu.add_item(conf_item);
            conf_item_menuid
        })
        .collect::<Vec<MenuId>>();

    main_tray.add_submenu("Profiles", true, conf_list_submenu);

    let tray_quit = MenuItemAttributes::new("Quit");
    main_tray.add_item(tray_quit);

    // TODO: have a nicer icon
    let icon = Icon::from_rgba(vec![70; 16], 2, 2).expect("error happen: ");

    let system_tray = SystemTrayBuilder::new(icon, Some(main_tray))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _event_loop, _control_flow| match event {
        tao::event::Event::MenuEvent { menu_id, .. } => {
            println!(
                "sth: {:?}. Is equal: later",
                menu_id,
                // menu_id == conf_item_menuid // TODO: hoist stuff out of loop to compare what was clicked
            );
        }
        _ => {}
    });
}
