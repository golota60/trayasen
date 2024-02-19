use tauri::AppHandle;

use crate::WindowInitUtils;

pub fn handle_exit_menu_click() {
    std::process::exit(0);
}

pub fn handle_about_menu_click(app: &AppHandle) {
    tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App("index.html".into()))
        .init_trayasen(
            "Trayasen - About/Options",
            "Error while trying to open about window",
            Some(
                r#"
    history.replaceState({}, '','/about');
    "#,
            ),
        );
}

pub fn handle_new_position_menu_click(app: &AppHandle) {
    tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App("index.html".into()))
        .init_trayasen(
            "Trayasen - Add position",
            "Error while trying to open new postition window",
            Some(
                r#"
    history.replaceState({}, '','/new-position');
    "#,
            ),
        );
}

pub fn handle_manage_positions_menu_click(app: &AppHandle) {
    tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App("index.html".into()))
        .init_trayasen(
            "Trayasen - Manage positions",
            "Error while trying to open manage positions window",
            Some(
                r#"
    history.replaceState({}, '','/manage-positions');
    "#,
            ),
        );
}
