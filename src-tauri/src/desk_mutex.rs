// Set of utils to interact with desk mutex, since it's pretty complex

use btleplug::platform::Peripheral as PlatformPeripheral;
use tauri::Manager;

use crate::{loose_idasen::BtError, TauriSharedDesk};

pub fn get_desk_from_app_state(app_handle: &tauri::AppHandle) -> PlatformPeripheral {
    let desk = app_handle.state::<TauriSharedDesk>();
    let desk = desk.0.lock().expect("Error while unwrapping shared desk");
    let desk = desk
        .as_ref()
        .expect("Desk should have been defined at this point");
    desk.clone()
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
