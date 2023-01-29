use btleplug::api::BDAddr;
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::{
    fs::{self, read_to_string, OpenOptions},
    io::Write,
    path::Path,
    process::Command,
};
use tao::{
    menu::{ContextMenu, MenuId, MenuItemAttributes},
    system_tray::Icon,
};

static CONFIG_FILE_NAME: &str = "idasen-tray-config.json";
static LINUX_DATA_DIR: &str = "$HOME/.local/share";
static MACOS_DATA_DIR: &str = "$HOME/Library\\ Application Support/";

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

enum SupportedSystems {
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

pub struct MenuConfigElem {
    pub elem_menu_id: MenuId,
    pub name: String,
    pub value: u16,
    pub conf_item_title: String,
}

pub fn get_menus_from_config(config: &ConfigData) -> Vec<MenuConfigElem> {
    config
        .saved_positions
        .iter()
        .map(|temp_conf_elem| {
            // Assign values so that they are not lost - TODO: figure out why the fuck does that even happen
            let name = &temp_conf_elem.name;
            let value = &temp_conf_elem.value;
            let conf_item_title = name.as_str().clone();
            let conf_item_menuid = MenuId::new(conf_item_title);
            MenuConfigElem {
                elem_menu_id: conf_item_menuid.clone(),
                name: name.clone(),
                value: value.clone(),
                conf_item_title: conf_item_title.clone().to_owned(),
            }
        })
        .collect::<Vec<MenuConfigElem>>()
}

pub fn recreate_submenu(args: &Vec<MenuConfigElem>, mut submenu: ContextMenu) -> ContextMenu {
    for el in args {
        let conf_item_button =
            MenuItemAttributes::new(&el.conf_item_title).with_id(el.elem_menu_id);
        submenu.add_item(conf_item_button);
    }
    submenu
}

pub struct MainTrayData {
    pub menu: ContextMenu,
    pub menu_ids: Vec<MenuConfigElem>,
    pub tray_quit_id: MenuId,
    pub tray_new_id: MenuId,
}

pub fn create_main_tray(config: &ConfigData) -> MainTrayData {
    let mut main_tray = ContextMenu::new();
    let mut conf_list_submenu = ContextMenu::new();

    // Header - does nothing, this is more of a decorator(a disabled button)
    let tray_header_button = MenuItemAttributes::new("Idasen controller").with_enabled(false);
    main_tray.add_item(tray_header_button);

    let tray_new_title = "Add a new position";
    let tray_new_id = MenuId::new("New position");
    let tray_new_button = MenuItemAttributes::new(tray_new_title).with_id(tray_new_id);
    conf_list_submenu.add_item(tray_new_button);

    let menu_ids = get_menus_from_config(&config);
    let conf_list_submenu = recreate_submenu(&menu_ids, conf_list_submenu);

    main_tray.add_submenu("Positions", true, conf_list_submenu);

    let tray_quit_id = MenuId::new("Quit");
    let tray_quit_button = MenuItemAttributes::new("Quit").with_id(tray_quit_id);
    main_tray.add_item(tray_quit_button);
    MainTrayData {
        menu: main_tray,
        menu_ids,
        tray_quit_id,
        tray_new_id,
    }
}

// https://github.com/tauri-apps/tao/blob/e1149563b85eb6187f5aa78d53cab9c5d7b87025/examples/system_tray.rs#L136
pub fn load_icon(path: &Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
