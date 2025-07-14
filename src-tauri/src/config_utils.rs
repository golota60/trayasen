use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::{
    fs::{self, read_to_string, remove_file, OpenOptions},
    io::Write,
};
use tauri::{Manager, path::BaseDirectory};
use tauri_plugin_global_shortcut::GlobalShortcutExt;

static CONFIG_FILE_NAME: &str = "idasen-tray-config.json";

pub const QUIT_ID: &str = "quit";
pub const ABOUT_ID: &str = "about/options";
pub const ADD_POSITION_ID: &str = "add_position";
pub const HEADER_ID: &str = "idasen_controller";
pub const MANAGE_POSITIONS_ID: &str = "manage_positions";

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Position {
    pub name: String,
    pub value: u16,
    /// String representation of shortcut
    pub shortcut: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConfigData {
    pub local_name: Option<String>,
    pub saved_positions: Vec<Position>,
}

fn get_config_path<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<std::path::PathBuf> {
    let path = app.path().resolve(CONFIG_FILE_NAME, BaseDirectory::AppData)?;
    Ok(path)
}

// TODO: use get_config here? or merge two funcs together?
// For FIRST loading
pub fn get_or_create_config<R: tauri::Runtime>(app: &tauri::AppHandle<R>) -> ConfigData {
    let config_path = get_config_path(app).expect("Error getting config path");

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
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent).expect("failed to create config directory");
            }
            let new_config = ConfigData {
                local_name: None,
                saved_positions: vec![],
            };
            let stringified_config = to_string::<ConfigData>(&new_config).unwrap();
            // Using OpenOptions cause it's the easiest to create a file with.
            let mut conf_file = OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .open(&config_path)
                .expect("Error while creating a new config");

            conf_file.write_all(&stringified_config.as_bytes()).unwrap();

            new_config
        }
    };

    config
}

// Generally this function should never error, cause all the same operations have been done miliseconds before.
pub fn save_local_name<R: tauri::Runtime>(app: &tauri::AppHandle<R>, new_local_name: String) {
    let config_path = get_config_path(app).expect("Error getting config path");
    let old_conf_file =
        read_to_string(&config_path).expect("Opening a config to save MAC Address");
    let mut mut_conf_file =
        from_str::<ConfigData>(&old_conf_file).expect("Parsing a config to save MAC Address");

    mut_conf_file.local_name = Some(new_local_name.to_string());

    let stringified_new_config = to_string::<ConfigData>(&mut_conf_file).unwrap();
    fs::write(config_path, stringified_new_config)
        .expect("Saving a config after parsing a MAC Address");
}

#[tauri::command]
pub fn remove_position(app_handle: tauri::AppHandle, pos_name: &str) -> ConfigData {
    let mut conf = get_config(app_handle.clone());

    let elem_to_unregister = conf.saved_positions.iter().find(|pos| pos_name == pos.name);

    if let Some(elem_to_unregister) = elem_to_unregister {
        let shortcut = elem_to_unregister.shortcut.clone();
        if let Some(shortcut) = shortcut {
            if shortcut != "" {
                _ = app_handle.global_shortcut().unregister(shortcut.as_str());
            }
        }
    }

    let new_conf_positions = conf
        .saved_positions
        .into_iter()
        .filter(|pos| pos.name != pos_name)
        .collect();
    conf.saved_positions = new_conf_positions;

    update_config(&app_handle, &conf);
    conf
}

#[tauri::command]
pub fn get_config(app_handle: tauri::AppHandle) -> ConfigData {
    let config_path = get_config_path(&app_handle).expect("Error getting config path");

    let old_conf_file = read_to_string(&config_path).expect("Opening a config");
    let stringified_new_config =
        from_str::<ConfigData>(&old_conf_file).expect("Parsing opened config to struct");

    stringified_new_config
}

pub fn update_config<R: tauri::Runtime>(app: &tauri::AppHandle<R>, updated_config: &ConfigData) {
    let config_path = get_config_path(app).expect("Error getting config path");

    let stringified_new_config = to_string::<ConfigData>(&updated_config).unwrap();
    fs::write(config_path, stringified_new_config)
        .expect("Saving a config after updating a config");
}

#[tauri::command]
pub fn remove_config(app_handle: tauri::AppHandle) {
    let config_path = get_config_path(&app_handle).expect("Error getting config path");

    let _ = remove_file(config_path);
}

#[tauri::command]
pub fn reset_desk(app_handle: tauri::AppHandle) {
    let config_path = get_config_path(&app_handle).expect("Error getting config path");

    let config =
        read_to_string(&config_path).expect("err while reading config while resetting desk");
    // Config exists
    let config = from_str::<ConfigData>(config.as_str()).expect("Error while parsing config file");

    let updated_config = ConfigData {
        local_name: None,
        saved_positions: config.saved_positions,
    };

    let stringified_new_config = to_string::<ConfigData>(&updated_config).unwrap();
    fs::write(config_path, stringified_new_config)
        .expect("Saving a config after updating a config");
}

pub struct MenuConfigItem<R: tauri::Runtime> {
    pub position_elem: tauri::menu::MenuItem<R>,
    pub name: String,
    pub value: u16,
    pub conf_item_title: String,
}

pub fn get_menu_items_from_config<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    config: &ConfigData,
) -> tauri::Result<Vec<MenuConfigItem<R>>> {
    let mut items = Vec::new();
    for temp_conf_elem in &config.saved_positions {
        let name = &temp_conf_elem.name;
        let conf_item_title = name.as_str();
        let position_elem = tauri::menu::MenuItem::with_id(app, conf_item_title, conf_item_title, true, None::<&str>)?;
        items.push(MenuConfigItem {
            position_elem,
            name: name.clone(),
            value: temp_conf_elem.value,
            conf_item_title: conf_item_title.to_string(),
        });
    }
    Ok(items)
}

/**
Utility function returning the tray menu instance, based on the provided config
*/
pub fn create_main_tray_menu<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    config: &ConfigData,
) -> tauri::Result<tauri::menu::Menu<R>> {
    let add_position_item = tauri::menu::MenuItem::with_id(
        app,
        ADD_POSITION_ID,
        "Add a new position",
        true,
        None::<&str>,
    )?;
    let manage_positions_item = tauri::menu::MenuItem::with_id(
        app,
        MANAGE_POSITIONS_ID,
        "Manage positions",
        true,
        None::<&str>,
    )?;
    
    let position_menu_items = get_menu_items_from_config(app, config)?;
    let position_menu_refs: Vec<&dyn tauri::menu::IsMenuItem<R>> = position_menu_items.iter().map(|item| &item.position_elem as &dyn tauri::menu::IsMenuItem<R>).collect();
    let positions_submenu = tauri::menu::Submenu::with_id_and_items(app, "positions", "Positions", true, &position_menu_refs)?;

    let header_item = tauri::menu::MenuItem::with_id(app, HEADER_ID, "Idasen Controller", false, None::<&str>)?;
    let about_item = tauri::menu::MenuItem::with_id(app, ABOUT_ID, "About/Options", true, None::<&str>)?;
    let quit_item = tauri::menu::MenuItem::with_id(app, QUIT_ID, "Quit", true, None::<&str>)?;

    let separator = tauri::menu::PredefinedMenuItem::separator(app)?;
    let main_menu_items: Vec<&dyn tauri::menu::IsMenuItem<R>> = vec![
        &add_position_item,
        &manage_positions_item,
        &separator,
        &positions_submenu,
        &separator,
        &header_item,
        &separator,
        &about_item,
        &quit_item,
    ];
    let main_menu = tauri::menu::Menu::with_items(app, &main_menu_items)?;

    Ok(main_menu)
}
