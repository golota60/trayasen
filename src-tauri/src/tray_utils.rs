use tauri::AppHandle;
use window_shadows::set_shadow;

pub fn handle_exit_menu_click() {
    std::process::exit(0);
}

pub fn handle_about_menu_click(app: &AppHandle) {
    match tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
        .always_on_top(true)
        .decorations(false)
        .initialization_script(
            r#"
    history.replaceState({}, '','/about');
    "#,
        )
        .title("Trayasen - About/Options")
        .build()
    {
        Ok(window) => {
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).unwrap();
        }
        Err(_) => {
            println!("Error while trying to open about window");
        }
    }
}

pub fn handle_new_position_menu_click(app: &AppHandle) {
    match tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
        .always_on_top(true)
        .decorations(false)
        .initialization_script(
            r#"
    history.replaceState({}, '','/new-position');
    "#,
        )
        .title("Trayasen - Add position")
        .build()
    {
        Ok(window) => {
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).unwrap();
        }
        Err(_) => {
            println!("Error while trying to open new postition window");
        }
    }
}

pub fn handle_manage_positions_menu_click(app: &AppHandle) {
    match tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("index.html".into()))
        .always_on_top(true)
        .decorations(false)
        .initialization_script(
            r#"
    history.replaceState({}, '','/manage-positions');
    "#,
        )
        .title("Trayasen - Manage positions")
        .build()
    {
        Ok(window) => {
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).unwrap();
        }
        Err(_) => {
            println!("Error while trying to open manage positions window");
        }
    }
}
