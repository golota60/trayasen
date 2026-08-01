use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    AppHandle, Manager,
};
use tauri_plugin_global_shortcut::GlobalShortcutExt;

static CONFIG_FILE_NAME: &str = "idasen-tray-config.json";
const POSITION_MENU_ID_PREFIX: &str = "position:";

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

#[derive(Debug)]
pub struct StartupConfig {
    pub config: ConfigData,
    pub recovery_error: Option<String>,
}

fn config_path_in(data_dir: PathBuf) -> PathBuf {
    data_dir.join(CONFIG_FILE_NAME)
}

fn get_config_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    app_handle
        .path()
        .data_dir()
        .map(config_path_in)
        .map_err(|error| format!("Could not resolve the config data directory: {error}"))
}

fn empty_config() -> ConfigData {
    ConfigData {
        local_name: None,
        saved_positions: vec![],
    }
}

fn parse_config(path: &Path, contents: &str) -> Result<ConfigData, String> {
    from_str::<ConfigData>(contents)
        .map_err(|error| format!("Could not parse config `{}`: {error}", path.display()))
}

fn read_config_at(path: &Path) -> Result<ConfigData, String> {
    let contents = fs::read_to_string(path)
        .map_err(|error| format!("Could not read config `{}`: {error}", path.display()))?;
    parse_config(path, &contents)
}

fn write_config_at(path: &Path, config: &ConfigData) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("Config path `{}` has no parent directory", path.display()))?;
    fs::create_dir_all(parent).map_err(|error| {
        format!(
            "Could not create config directory `{}`: {error}",
            parent.display()
        )
    })?;
    let contents =
        to_string(config).map_err(|error| format!("Could not serialize config: {error}"))?;
    fs::write(path, contents)
        .map_err(|error| format!("Could not write config `{}`: {error}", path.display()))
}

fn get_or_create_config_at(path: &Path) -> Result<ConfigData, String> {
    match fs::read_to_string(path) {
        Ok(contents) => parse_config(path, &contents),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            let config = empty_config();
            write_config_at(path, &config)?;
            Ok(config)
        }
        Err(error) => Err(format!(
            "Could not read config `{}`: {error}",
            path.display()
        )),
    }
}

pub fn get_or_create_config(app_handle: &AppHandle) -> Result<ConfigData, String> {
    let config_path = get_config_path(app_handle)?;
    println!("Config path: {:?}", config_path);
    get_or_create_config_at(&config_path)
}

fn startup_config_at(path: &Path) -> StartupConfig {
    match get_or_create_config_at(path) {
        Ok(config) => StartupConfig {
            config,
            recovery_error: None,
        },
        Err(error) => StartupConfig {
            config: empty_config(),
            recovery_error: Some(error),
        },
    }
}

pub fn startup_config(app_handle: &AppHandle) -> StartupConfig {
    match get_config_path(app_handle) {
        Ok(config_path) => {
            println!("Config path: {:?}", config_path);
            startup_config_at(&config_path)
        }
        Err(error) => StartupConfig {
            config: empty_config(),
            recovery_error: Some(error),
        },
    }
}

pub fn save_local_name(app_handle: &AppHandle, new_local_name: String) -> Result<(), String> {
    let config_path = get_config_path(app_handle)?;
    let mut config = read_config_at(&config_path)?;
    config.local_name = Some(new_local_name);
    write_config_at(&config_path, &config)
}

#[tauri::command]
pub fn remove_position(app_handle: tauri::AppHandle, pos_name: &str) -> Result<ConfigData, String> {
    let mut conf = get_config(app_handle.clone())?;

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
    update_config(&app_handle, &conf)?;
    Ok(conf)
}

#[tauri::command]
pub fn get_config(app_handle: tauri::AppHandle) -> Result<ConfigData, String> {
    let config_path = get_config_path(&app_handle)?;
    read_config_at(&config_path)
}

pub fn update_config(app_handle: &AppHandle, updated_config: &ConfigData) -> Result<(), String> {
    let config_path = get_config_path(app_handle)?;
    write_config_at(&config_path, updated_config)
}

#[tauri::command]
pub fn remove_config(app_handle: tauri::AppHandle) -> Result<(), String> {
    let config_path = get_config_path(&app_handle)?;
    match fs::remove_file(&config_path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!(
            "Could not remove config `{}`: {error}",
            config_path.display()
        )),
    }
}

#[tauri::command]
pub fn reset_desk(app_handle: tauri::AppHandle) -> Result<(), String> {
    let config_path = get_config_path(&app_handle)?;
    let config = read_config_at(&config_path)?;
    let updated_config = ConfigData {
        local_name: None,
        saved_positions: config.saved_positions,
    };
    write_config_at(&config_path, &updated_config)
}

pub fn position_menu_id(position_name: &str) -> String {
    format!("{POSITION_MENU_ID_PREFIX}{position_name}")
}

pub fn position_name_from_menu_id(menu_id: &str) -> Option<&str> {
    menu_id.strip_prefix(POSITION_MENU_ID_PREFIX)
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
                position_menu_id(&position.name),
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

    #[test]
    fn position_ids_do_not_collide_with_fixed_actions() {
        assert_eq!(position_menu_id("about"), "position:about");
        assert_eq!(position_name_from_menu_id("position:about"), Some("about"));
        assert_eq!(position_name_from_menu_id(ABOUT_ID), None);
    }

    #[test]
    fn missing_config_creates_parent_directories() {
        let root = std::env::temp_dir().join(format!(
            "trayasen-config-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let path = root.join("nested").join(CONFIG_FILE_NAME);

        let config = get_or_create_config_at(&path).unwrap();

        assert_eq!(config.local_name, None);
        assert!(config.saved_positions.is_empty());
        assert!(path.is_file());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn malformed_config_enters_recovery_without_overwriting_it() {
        let root = std::env::temp_dir().join(format!(
            "trayasen-malformed-config-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let path = root.join(CONFIG_FILE_NAME);
        fs::write(&path, "not json").unwrap();

        let startup = startup_config_at(&path);

        assert!(startup.recovery_error.unwrap().contains("parse"));
        assert_eq!(startup.config.local_name, None);
        assert!(startup.config.saved_positions.is_empty());
        assert_eq!(fs::read_to_string(&path).unwrap(), "not json");
        fs::remove_dir_all(root).unwrap();
    }
}
