use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

use crate::{route_initialization_script, WindowInitUtils};

pub fn handle_exit_menu_click() {
    std::process::exit(0);
}

fn focus_window(window: &WebviewWindow, title: &str) -> tauri::Result<()> {
    window.set_title(title)?;
    window.show()?;
    window.unminimize()?;
    window.set_focus()?;
    Ok(())
}

fn navigate_window(window: &WebviewWindow, route: &str) -> tauri::Result<()> {
    let script = format!(
        "{}\nwindow.dispatchEvent(new PopStateEvent('popstate'));",
        route_initialization_script(route)
    );
    window.eval(script)
}

fn open_app_route(app: &AppHandle, route: &str, title: &str) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window("main") {
        navigate_window(&window, route)?;
        focus_window(&window, title)?;
        return Ok(());
    }

    let init_script = route_initialization_script(route);
    let window = WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
        .init_trayasen(title, Some(&init_script))?;
    focus_window(&window, title)
}

fn open_app_route_or_log(app: &AppHandle, route: &str, title: &str) {
    if let Err(error) = open_app_route(app, route, title) {
        eprintln!("Could not open Trayasen route `{route}`: {error}");
    }
}

pub fn handle_about_menu_click(app: &AppHandle) {
    open_app_route_or_log(app, "/about", "Trayasen - About/Options");
}

pub fn handle_new_position_menu_click(app: &AppHandle) {
    open_app_route_or_log(app, "/new-position", "Trayasen - Add position");
}

pub fn handle_manage_positions_menu_click(app: &AppHandle) {
    open_app_route_or_log(app, "/manage-positions", "Trayasen - Manage positions");
}
