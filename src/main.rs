use idasen::get_instance_by_mac;
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
    config
}

// EE:4D:A2:34:E4:8F
fn main() {
    let rt = tokio::runtime::Runtime::new().expect("Error while initializing runtime");
    gtk::init().expect("Error while initializing GTK");

    let path = load_config();
    println!("{:?}", path);

    // let desk = rt.spawn();
    let desk = rt.block_on(async { get_instance_by_mac("EE:4D:A2:34:E4:8F").await });
    // TODO: Find a better way of unwrapping this shit
    let desk = match desk {
        Ok(desk) => desk,
        Err(error) => panic!("Error while connecting to the desk: {:?}", error),
    };

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
