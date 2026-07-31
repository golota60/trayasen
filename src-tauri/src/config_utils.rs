use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::{
    fs::{self, read_to_string, remove_file, OpenOptions},
    io::Write,
    path::PathBuf,
};
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    AppHandle, Manager,
};
use tauri_plugin_global_shortcut::GlobalShortcutExt;

static CONFIG_FILE_NAME: &str = "idasen-tray-config.json";

pub const QUIT_ID: &str = "quit";
pub const ABOUT_ID: &str = "about";
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

fn config_path_in(data_dir: PathBuf) -> PathBuf {
    data_dir.join(CONFIG_FILE_NAME)
}

fn get_config_path(app_handle: &AppHandle) -> PathBuf {
    config_path_in(
        app_handle
            .path()
            .data_dir()
            .expect("Error while unwrapping data directory"),
    )
}

// TODO: use get_config here? or merge two funcs together?
// For FIRST loading
pub fn get_or_create_config(app_handle: &AppHandle) -> ConfigData {
    let config_path = get_config_path(app_handle);

    println!("Config path: {:?}", config_path);

    match read_to_string(&config_path) {
        // Config exists
        Ok(s) => from_str::<ConfigData>(s.as_str()).expect("Error while parsing config file"),
        // Config does not exist. Create a dummy one.
        // Check for different errors?
        Err(_) => {
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

            conf_file.write_all(stringified_config.as_bytes()).unwrap();

            new_config
        }
    }
}

// Generally this function should never error, cause all the same operations have been done miliseconds before.
pub fn save_local_name(app_handle: &AppHandle, new_local_name: String) {
    let config_path = get_config_path(app_handle);
    let old_conf_file = read_to_string(&config_path).expect("Opening a config to save MAC Address");
    let mut mut_conf_file =
        from_str::<ConfigData>(&old_conf_file).expect("Parsing a config to save MAC Address");

    mut_conf_file.local_name = Some(new_local_name);

    let stringified_new_config = to_string::<ConfigData>(&mut_conf_file).unwrap();
    fs::write(config_path, stringified_new_config)
        .expect("Saving a config after parsing a MAC Address");
}

#[tauri::command]
pub fn remove_position(app_handle: tauri::AppHandle, pos_name: &str) -> ConfigData {
    let mut conf = get_config(app_handle.clone());

    let elem_to_unregister = conf.saved_positions.iter().find(|pos| pos_name == pos.name);

    if let Some(elem_to_unregister) = elem_to_unregister {
        if let Some(shortcut) = &elem_to_unregister.shortcut {
            if !shortcut.is_empty() {
                if let Err(error) = app_handle.global_shortcut().unregister(shortcut.as_str()) {
                    eprintln!(
                        "Failed to unregister global shortcut `{shortcut}` for position `{pos_name}`: {error}"
                    );
                }
            }
        }
    }

    conf.saved_positions.retain(|pos| pos.name != pos_name);

    update_config(&app_handle, &conf);
    conf
}

#[tauri::command]
pub fn get_config(app_handle: tauri::AppHandle) -> ConfigData {
    let config_path = get_config_path(&app_handle);

    let old_conf_file = read_to_string(&config_path).expect("Opening a config");
    from_str::<ConfigData>(&old_conf_file).expect("Parsing opened config to struct")
}

pub fn update_config(app_handle: &AppHandle, updated_config: &ConfigData) {
    let config_path = get_config_path(app_handle);

    let stringified_new_config = to_string::<ConfigData>(updated_config).unwrap();
    fs::write(config_path, stringified_new_config)
        .expect("Saving a config after updating a config");
}

#[tauri::command]
pub fn remove_config(app_handle: tauri::AppHandle) {
    let config_path = get_config_path(&app_handle);

    let _ = remove_file(config_path);
}

#[tauri::command]
pub fn reset_desk(app_handle: tauri::AppHandle) {
    let config_path = get_config_path(&app_handle);

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

pub struct MenuConfigItem {
    pub position_elem: MenuItem<tauri::Wry>,
}

pub fn get_menu_items_from_config(
    app_handle: &AppHandle,
    config: &ConfigData,
) -> tauri::Result<Vec<MenuConfigItem>> {
    config
        .saved_positions
        .iter()
        .map(|position| {
            let position_elem = MenuItem::with_id(
                app_handle,
                position.name.clone(),
                &position.name,
                true,
                None::<&str>,
            )?;
            Ok(MenuConfigItem { position_elem })
        })
        .collect()
}

/**
Utility function returning the tray menu instance, based on the provided config
*/
pub fn create_main_tray_menu(
    app_handle: &AppHandle,
    config: &ConfigData,
) -> tauri::Result<Menu<tauri::Wry>> {
    let add_position_item = MenuItem::with_id(
        app_handle,
        ADD_POSITION_ID,
        "Add a new position",
        true,
        None::<&str>,
    )?;
    let manage_positions_item = MenuItem::with_id(
        app_handle,
        MANAGE_POSITIONS_ID,
        "Manage positions",
        true,
        None::<&str>,
    )?;
    let position_menu_items = get_menu_items_from_config(app_handle, config)?;

    let positions_submenu = Submenu::new(app_handle, "Positions", true)?;
    positions_submenu.append(&add_position_item)?;
    positions_submenu.append(&manage_positions_item)?;
    positions_submenu.append(&PredefinedMenuItem::separator(app_handle)?)?;
    for item in &position_menu_items {
        positions_submenu.append(&item.position_elem)?;
    }

    let header_item = MenuItem::with_id(
        app_handle,
        HEADER_ID,
        "Idasen Controller",
        false,
        None::<&str>,
    )?;
    let about_item = MenuItem::with_id(app_handle, ABOUT_ID, "About/Options", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app_handle, QUIT_ID, "Quit", true, None::<&str>)?;

    let main_menu = Menu::new(app_handle)?;
    main_menu.append(&header_item)?;
    main_menu.append(&PredefinedMenuItem::separator(app_handle)?)?;
    main_menu.append(&positions_submenu)?;
    main_menu.append(&PredefinedMenuItem::separator(app_handle)?)?;
    main_menu.append(&about_item)?;
    main_menu.append(&quit_item)?;

    Ok(main_menu)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_filename_remains_compatible() {
        assert_eq!(
            config_path_in(PathBuf::from("data")),
            PathBuf::from("data").join("idasen-tray-config.json")
        );
    }

    #[test]
    fn tray_action_ids_remain_stable() {
        assert_eq!(ADD_POSITION_ID, "add_position");
        assert_eq!(MANAGE_POSITIONS_ID, "manage_positions");
        assert_eq!(ABOUT_ID, "about");
        assert_eq!(QUIT_ID, "quit");
    }
}
