use idasen::get_instance_by_mac;
use serde_derive::{Deserialize, Serialize};
use std::process::Command;
use std::str;
use tray_item::TrayItem;

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
    gtk::init().expect("Error while initializing GTK");

    let config_path = load_config();
    println!("{:?}", config_path);

    let config = {

        let file = std::fs::read_to_string(config_path).expect("Error while reading config file");

        serde_json::from_str::<ConfigData>(&file).expect("Error while parsing config file")
    };

    println!("Config: {:?}", config);

    let desk = rt.block_on(async { get_instance_by_mac("EE:4D:A2:34:E4:8F").await });
    let desk = desk.expect("Error while connecting to the desk:");

    // TODO: how accessorries icons work?
    let mut tray = TrayItem::new("Idasen desk tray", "accessories-calculator").unwrap();

    tray.add_label("Idasen controller").unwrap();

    tray.add_menu_item("Bring up", move || {
        rt.block_on(async {
            println!("Trying to bring your idasen up!");
            desk.move_to(8000).await.unwrap();
        });
    })
    .unwrap();

    tray.add_menu_item("Quit", || {
        gtk::main_quit();
    })
    .unwrap();

    gtk::main();
}
