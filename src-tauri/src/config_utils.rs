use btleplug::api::BDAddr;
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::{
    fs::{self, read_to_string, OpenOptions},
    io::Write,
    process::Command,
};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu};

static CONFIG_FILE_NAME: &str = "idasen-tray-config.json";
static LINUX_DATA_DIR: &str = "$HOME/.local/share";
static MACOS_DATA_DIR: &str = "$HOME/Library\\ Application Support/";

pub const QUIT_ID: &str = "quit";
pub const ABOUT_ID: &str = "about";
pub const ADD_POSITION_ID: &str = "add_position";
pub const HEADER_ID: &str = "idasen_controller";

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Position {
    pub name: String,
    pub value: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConfigData {
    pub mac_address: Option<String>,
    pub saved_positions: Vec<Position>,
}

pub enum SupportedSystems {
    Linux,
    MacOS,
    Windows,
}

impl SupportedSystems {
    fn get_config_path(&self) -> String {
        match self {
            SupportedSystems::Linux => {
                let path = format!("echo {}/{}", LINUX_DATA_DIR, CONFIG_FILE_NAME);
                let home = Command::new("sh")
                    .arg("-c")
                    .arg(path)
                    .output()
                    .expect("Failed to get config path");

                String::from_utf8(home.stdout).expect("Error while parsing config path")
            }
            SupportedSystems::MacOS => {
                let path = format!("echo {}/{}", MACOS_DATA_DIR, CONFIG_FILE_NAME);
                let home = Command::new("sh")
                    .arg("-c")
                    .arg(path)
                    .output()
                    .expect("Failed to get config path");

                String::from_utf8(home.stdout).expect("Error while parsing config path")
                // todo!()
            }
            SupportedSystems::Windows => {
                // {FOLDERID_RoamingAppData}
                todo!()
            }
        }
    }
}

pub fn load_config() -> ConfigData {
    let config_path = SupportedSystems::Linux
        .get_config_path()
        .trim_end()
        .to_string();

    println!("Config path: {:?}", config_path);

    let config = match read_to_string(&config_path) {
        // Config exists
        Ok(s) => {
            let config =
                from_str::<ConfigData>(s.as_str()).expect("Error while parsing config file");
            config
        }
        // Config does not exist. Create a dummy one.
        // Check for different errors?
        Err(_) => {
            let new_config = ConfigData {
                mac_address: None,
                saved_positions: vec![],
            };
            let stringified_config = to_string::<ConfigData>(&new_config).unwrap();
            // Using OpenOptions cause it's the easiest to create a file with.
            let mut conf_file = OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .open(&config_path.to_string())
                .expect("Error while creating a new config");

            conf_file.write_all(&stringified_config.as_bytes()).unwrap();

            new_config
        }
    };

    config
}

// Generally this function should never error, cause all the same operations have been done miliseconds before.
pub fn save_mac_address(new_mac_address: BDAddr) {
    let config_path = SupportedSystems::Linux
        .get_config_path()
        .trim_end()
        .to_string();
    let old_conf_file =
        read_to_string(&config_path.to_string()).expect("Opening a config to save MAC Address");
    let mut mut_conf_file =
        from_str::<ConfigData>(&old_conf_file).expect("Parsing a config to save MAC Address");

    mut_conf_file.mac_address = Some(new_mac_address.to_string());

    let stringified_new_config = to_string::<ConfigData>(&mut_conf_file).unwrap();
    fs::write(config_path, stringified_new_config)
        .expect("Saving a config after parsing a MAC Address");
}

pub fn get_config() -> ConfigData {
    let config_path = SupportedSystems::Linux
        .get_config_path()
        .trim_end()
        .to_string();

    let old_conf_file = read_to_string(&config_path.to_string()).expect("Opening a config");
    let stringified_new_config =
        from_str::<ConfigData>(&old_conf_file).expect("Parsing opened config to struct");

    stringified_new_config
}

pub fn update_config(updated_config: &ConfigData) {
    let config_path = SupportedSystems::Linux
        .get_config_path()
        .trim_end()
        .to_string();

    let stringified_new_config = to_string::<ConfigData>(&updated_config).unwrap();
    fs::write(config_path, stringified_new_config)
        .expect("Saving a config after updatign a config");
}

pub struct MenuConfigItem {
    pub position_elem: CustomMenuItem,
    pub name: String,
    pub value: u16,
    pub conf_item_title: String,
}

pub fn get_menu_items_from_config(config: &ConfigData) -> Vec<MenuConfigItem> {
    config
        .saved_positions
        .iter()
        .map(|temp_conf_elem| {
            // Assign values so that they are not lost - TODO: figure out why the fuck does that even happen
            let name = &temp_conf_elem.name;
            let value = &temp_conf_elem.value;
            let conf_item_title = name.as_str().clone();
            let position_elem = CustomMenuItem::new(conf_item_title, conf_item_title);
            MenuConfigItem {
                position_elem: position_elem.clone(),
                name: name.clone(),
                value: value.clone(),
                conf_item_title: conf_item_title.clone().to_owned(),
            }
        })
        .collect::<Vec<MenuConfigItem>>()
}

// pub fn get_position_items(args: &Vec<MenuConfigItem>) -> CustomMenuItem {
//     //     let x = args
//     //     .iter()
//     //     .map(|el| {
//     //         // TODO: this might be fucked
//     //         let conf_item_button =
//     //             CustomMenuItem::new(el.position_elem.id.to_string(), &el.position_elem.title);
//     //         conf_item_button
//     //     })
//     //     .collect::<Vec<&CustomMenuItem>>();
//     // x
//     for el in args {
//         let conf_item_button = CustomMenuItem::new(el.elem_menu_id, &el.conf_item_title);
//     }
//     conf_item_button
// }

pub fn create_main_tray(config: &ConfigData) -> SystemTray {
    let add_position_item = CustomMenuItem::new(ADD_POSITION_ID.to_string(), "Add a new position");
    let position_menu_items = get_menu_items_from_config(&config);
    // The element that opens up on hover

    let mut sys_tray_menu = SystemTrayMenu::new()
        .add_item(add_position_item.clone())
        .add_native_item(SystemTrayMenuItem::Separator);

    // Populate submenu
    for item in &position_menu_items {
        sys_tray_menu = sys_tray_menu.add_item(item.position_elem.clone());
    }

    // let x = sys_tray_menu.as_ref();
    // The element to show in the main_menu
    let positions_submenu = SystemTraySubmenu::new("Positions", sys_tray_menu);

    let header_item = CustomMenuItem::new(HEADER_ID.to_string(), "Idasen Controller").disabled();
    let about_item = CustomMenuItem::new(ABOUT_ID.to_string(), "About");
    let quit_item = CustomMenuItem::new(QUIT_ID.to_string(), "Quit");
    let main_menu = SystemTrayMenu::new()
        .add_item(header_item)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_submenu(positions_submenu)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(about_item)
        .add_item(quit_item.clone());

    let tray = SystemTray::new().with_menu(main_menu);
    tray
}
