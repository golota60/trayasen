// Set of utils to interact with desk mutex, since it's pretty complex

use btleplug::platform::Peripheral as PlatformPeripheral;
use tauri::Manager;

use crate::{loose_idasen::BtError, TauriSharedDesk};

pub fn get_desk_from_app_state(
    app_handle: &tauri::AppHandle,
) -> Result<PlatformPeripheral, String> {
    let desk = app_handle.state::<TauriSharedDesk>();
    let desk = desk
        .0
        .lock()
        .map_err(|_| "Could not lock shared desk state".to_string())?;
    desk.as_ref()
        .cloned()
        .map_err(|error| format!("Desk is not ready: {error}"))
}

pub fn assign_desk_to_mutex(
    desk_mutex: &TauriSharedDesk,
    new_desk: Result<PlatformPeripheral, BtError>,
) {
    *desk_mutex
        .0
        .lock()
        .expect("Failed to deref mutex during instantiation") = new_desk;
}
