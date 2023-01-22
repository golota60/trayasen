use btleplug::api::BDAddr;
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::{
    fs::{self, read_to_string, OpenOptions},
    io::Write,
    path::Path,
    process::Command,
};
use tao::system_tray::Icon;

static CONFIG_FILE_NAME: &str = "idasen-tray-config.json";
static LINUX_DATA_DIR: &str = "$HOME/.local/share";
static MACOS_DATA_DIR: &str = "$HOME/Library\\ Application Support/";

#[derive(Deserialize, Serialize, Debug, Clone)]
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

pub fn update_config(updated_config: ConfigData) {
    let config_path = SupportedSystems::Linux
        .get_config_path()
        .trim_end()
        .to_string();

    let stringified_new_config = to_string::<ConfigData>(&updated_config).unwrap();
    fs::write(config_path, stringified_new_config)
        .expect("Saving a config after updatign a config");
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
