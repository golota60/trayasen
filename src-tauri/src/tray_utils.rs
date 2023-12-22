use tauri::{AppHandle, Manager};

pub fn handle_exit_menu_click() {
    std::process::exit(0);
}

pub fn handle_about_menu_click(app: &AppHandle) {
    let main_window = app.get_window("main").unwrap();

    _ = main_window.set_always_on_top(true);
    _ = main_window.eval(
        r#"
        history.replaceState({}, '','/about');
        history.go();
"#,
    );
    _ = main_window.set_title("Trayasen - About/Options");
    _ = main_window.show();
}

pub fn handle_new_position_menu_click(app: &AppHandle) {
    let main_window = app.get_window("main").unwrap();

    _ = main_window.set_always_on_top(true);
    _ = main_window.eval(
        r#"
        history.replaceState({}, '','/new-position');
        history.go();
"#,
    );
    _ = main_window.set_title("Trayasen - Add position");
    _ = main_window.show();
}

pub fn handle_manage_positions_menu_click(app: &AppHandle) {
    let main_window = app.get_window("main").unwrap();

    _ = main_window.set_always_on_top(true);
    _ = main_window.eval(
        r#"
    history.replaceState({}, '','/manage-positions');
    history.go();
"#,
    );
    _ = main_window.set_title("Trayasen - Manage positions");
    _ = main_window.show();
}
