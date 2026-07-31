use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};

use crate::{route_initialization_script, WindowInitUtils};

pub fn handle_exit_menu_click() {
    std::process::exit(0);
}

fn open_app_route(app: &AppHandle, route: &str, title: &str, error_message: &str) {
    let init_script = route_initialization_script(route);
    WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into())).init_trayasen(
        title,
        error_message,
        Some(&init_script),
    );
}

pub fn handle_about_menu_click(app: &AppHandle) {
    open_app_route(
        app,
        "/about",
        "Trayasen - About/Options",
        "Error while trying to open about window",
    );
}

pub fn handle_new_position_menu_click(app: &AppHandle) {
    open_app_route(
        app,
        "/new-position",
        "Trayasen - Add position",
        "Error while trying to open new position window",
    );
}

pub fn handle_manage_positions_menu_click(app: &AppHandle) {
    open_app_route(
        app,
        "/manage-positions",
        "Trayasen - Manage positions",
        "Error while trying to open manage positions window",
    );
}
