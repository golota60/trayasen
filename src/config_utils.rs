use serde_derive::{Deserialize, Serialize};
use serde_json::from_str;
use std::{fs::read_to_string, process::Command};

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
    let config_path = SupportedSystems::Linux.get_config_path();
    let config_path = config_path.trim_end().to_string();

    println!("Config path: {:?}", config_path);

    let config = {
        let file = read_to_string(config_path).expect("Error while reading config file");
        let config =
            from_str::<ConfigData>(file.as_str()).expect("Error while parsing config file");
        config
    };

    config
}
