use std::{
    cmp::Ordering,
    thread::sleep,
    time::{Duration, Instant},
};

use btleplug::{
    api::{
        BDAddr, Central, Characteristic, Manager as ApiManager, ParseBDAddrError,
        Peripheral as ApiPeripheral, ScanFilter, WriteType,
    },
    platform::{Adapter, Manager, Peripheral as PlatformPeripheral},
};
use serde::Serialize;
use uuid::Uuid;

use crate::{config_utils, TauriSharedDesk};

/*
  This file contains loose utils to interact with a desk bluetooth peripheral as if it's a desk.
  Requires the device in question to already be connected, otherwise it will error
*/

const CONTROL_UUID: Uuid = Uuid::from_bytes([
    0x99, 0xfa, 0x00, 0x02, 0x33, 0x8a, 0x10, 0x24, 0x8a, 0x49, 0x00, 0x9c, 0x02, 0x15, 0xf7, 0x8a,
]);
const POSITION_UUID: Uuid = Uuid::from_bytes([
    0x99, 0xfa, 0x00, 0x21, 0x33, 0x8a, 0x10, 0x24, 0x8a, 0x49, 0x00, 0x9c, 0x02, 0x15, 0xf7, 0x8a,
]);

const UP: [u8; 2] = [0x47, 0x00];
const DOWN: [u8; 2] = [0x46, 0x00];
const STOP: [u8; 2] = [0xFF, 0x00];

pub const MIN_HEIGHT: u16 = 6200;
pub const MAX_HEIGHT: u16 = 12700;

#[derive(Debug, PartialEq, Eq)]
pub struct PositionSpeed {
    // tenth mm
    pub position: u16,
    // unknown
    pub speed: i16,
}

pub fn bytes_to_position_speed(bytes: &[u8]) -> PositionSpeed {
    let position = u16::from_le_bytes([bytes[0], bytes[1]]) + MIN_HEIGHT;
    let speed = i16::from_le_bytes([bytes[2], bytes[3]]);
    PositionSpeed { position, speed }
}

#[derive(Debug, thiserror::Error)]
pub enum BtError {
    #[error("Cannot find the device.")]
    CannotFindDevice,

    #[error("Bluetooth characteristics not found: '{}'.", _0)]
    CharacteristicsNotFound(String),

    #[error("Desired position has to be between MIN_HEIGHT and MAX_HEIGHT.")]
    PositionNotInRange,

    #[error("Cannot subscribe to read position.")]
    CannotSubscribePosition,

    #[error("errored to parse mac address.")]
    MacAddrParseFailed(#[from] ParseBDAddrError),

    #[error("bluetooth error {0}")]
    BtlePlugError(#[from] btleplug::Error),
}

pub struct ConnectedBtDevice<T>
where
    T: ApiPeripheral,
{
    pub mac_addr: BDAddr,
    pub device_instance: T,
    pub control_characteristic: Characteristic,
    pub position_characteristic: Characteristic,
}

/// Do a set of tasks for a peripheral to make the device(desk) usable.
pub async fn setup_bt_desk_device(
    device: &impl ApiPeripheral,
) -> Result<ConnectedBtDevice<impl ApiPeripheral>, BtError> {
    let mac_addr = BDAddr::default();
    println!("got the mac! desk: {:?}", &device);
    device.connect().await.unwrap();
    device.discover_services().await.unwrap();

    let control_characteristic = get_control_characteristic(device).await;
    let position_characteristic = get_position_characteristic(device).await;

    if device.subscribe(&position_characteristic).await.is_err() {
        return Err(BtError::CannotSubscribePosition);
    };
    println!("Desk is fully set up");

    Ok(ConnectedBtDevice {
        device_instance: device.to_owned(),
        mac_addr,
        control_characteristic,
        position_characteristic,
    })
}

pub async fn get_list_of_desks(loc_name: &Option<String>) -> Vec<ExpandedPeripheral> {
    let desks = match loc_name {
        // If local name was provided
        Some(loc_name) => {
            let desks = get_desks(Some(loc_name.clone())).await;
            desks
        }
        // If local name was NOT provided
        None => {
            let desks = get_desks(None).await;
            desks
        }
    };
    let desks = desks.expect("Error while getting a list of desks");

    desks
}

// Getting characteristics every time is wasteful
// TODO: Try to refactor this - maybe chuck this into shared tauri state?
pub async fn get_control_characteristic(desk: &impl ApiPeripheral) -> Characteristic {
    desk.characteristics()
        .iter()
        .find(|c| c.uuid == CONTROL_UUID)
        .ok_or_else(|| BtError::CharacteristicsNotFound("Control".to_string()))
        .expect("err while getting characteristic")
        .clone()
}

pub async fn get_position_characteristic(desk: &impl ApiPeripheral) -> Characteristic {
    desk.characteristics()
        .iter()
        .find(|c| c.uuid == POSITION_UUID)
        .ok_or_else(|| BtError::CharacteristicsNotFound("Position".to_string()))
        .expect("Error while getting position characteristic")
        .clone()
}

async fn up(desk: &impl ApiPeripheral) -> btleplug::Result<()> {
    let control_characteristic = get_control_characteristic(desk).await;

    desk.write(&control_characteristic, &UP, WriteType::WithoutResponse)
        .await
}

async fn down(desk: &impl ApiPeripheral) -> btleplug::Result<()> {
    let control_characteristic = get_control_characteristic(desk).await;
    desk.write(&control_characteristic, &DOWN, WriteType::WithoutResponse)
        .await
}

async fn stop(desk: &impl ApiPeripheral) -> btleplug::Result<()> {
    let control_characteristic = get_control_characteristic(desk).await;
    desk.write(&control_characteristic, &STOP, WriteType::WithoutResponse)
        .await
}

pub async fn move_to_target(
    desk: &impl ApiPeripheral,
    target_position: u16,
) -> Result<(), BtError> {
    println!("starting moving to target");
    if !(MIN_HEIGHT..=MAX_HEIGHT).contains(&target_position) {
        return Err(BtError::PositionNotInRange);
    }

    let mut position_reached = false;
    let last_position = get_position(desk).await? as i16;
    let last_position_read_at = Instant::now();
    let target_position = target_position as i16;
    while !position_reached {
        sleep(Duration::from_millis(200));
        let current_position = get_position(desk).await? as i16;
        let going_up = match target_position.cmp(&current_position) {
            Ordering::Greater => true,
            Ordering::Less => false,
            Ordering::Equal => return Ok(()),
        };
        let remaining_distance = (target_position - current_position).abs();

        println!(
            "lastpos: {}, lastposreadat: {:?}, rem_dist: {}",
            last_position, last_position_read_at, remaining_distance
        );

        // If under/over 1cm we call it a day. From my testing it's under <3mm always(sometimes it might fuck up and do like 8mm but fuck it)
        if remaining_distance <= 100 {
            println!("position reached!");
            position_reached = true;
            stop(desk).await?;
        } else if going_up {
            up(desk).await?;
        } else if !going_up {
            down(desk).await?;
        }
    }

    Ok(())
}

pub async fn get_position(desk: &impl ApiPeripheral) -> Result<u16, BtError> {
    Ok(get_position_and_speed(desk).await?.position)
}

pub async fn get_position_and_speed(desk: &impl ApiPeripheral) -> Result<PositionSpeed, BtError> {
    let position_characteristic = get_position_characteristic(desk).await;

    let value = desk.read(&position_characteristic).await?;
    Ok(bytes_to_position_speed(&value))
}

/// Peripheral expanded with it's name(we treat it as an ID)
pub struct ExpandedPeripheral {
    pub perp: PlatformPeripheral,
    pub name: String,
}

pub async fn get_desks(loc_name: Option<String>) -> Result<Vec<ExpandedPeripheral>, BtError> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let mut jobs = Vec::new();

    for adapter in adapters {
        jobs.push(search_adapter_for_desks(adapter, loc_name.clone()).await);
    }

    let mut desks = Vec::new();
    for job in jobs {
        desks.append(&mut job.unwrap());
    }

    if desks.is_empty() {
        Err(BtError::CannotFindDevice)
    } else {
        Ok(desks)
    }
}

async fn search_adapter_for_desks(
    adapter: Adapter,
    name: Option<String>,
) -> Result<Vec<ExpandedPeripheral>, BtError> {
    adapter.start_scan(ScanFilter::default()).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    let mut desks = Vec::new();
    for peripheral in adapter.peripherals().await? {
        if let Some(props) = peripheral.properties().await? {
            if match name {
                Some(ref device_name) => {
                    // We're matching by name - ideally we'd do this by MAC Address, but macOS doesn't expose MAC addresses of bluetooth devices
                    let y = props.address;

                    // some devices might not have a local name
                    let name = props.local_name.clone().unwrap_or("".to_string());

                    device_name == &name
                }
                None => props.local_name.iter().any(|name| name.contains("")),
            } {
                desks.push(ExpandedPeripheral {
                    perp: peripheral,
                    name: props.local_name.unwrap_or("".to_string()),
                });
            }
        }
    }
    Ok(desks)
}

enum SavedDeskStates {
    New,
    Saved,
}

impl SavedDeskStates {
    fn as_str(&self) -> &'static str {
        match self {
            SavedDeskStates::New => "new",
            SavedDeskStates::Saved => "saved",
        }
    }
}

/// A type of a potential candidate to be a desk - essentially just a bluetooth device
#[derive(Serialize, Debug)]
pub struct PotentialDesk {
    pub name: String,
    pub status: String,
}
// https://github.com/tauri-apps/tauri/issues/2533 - this has to be a Result
/// Desk we're connecting to for UI info
#[tauri::command]
pub async fn get_desk_to_connect() -> Result<Vec<PotentialDesk>, ()> {
    let config = config_utils::get_or_create_config();
    let desk_list = get_list_of_desks(&config.local_name).await;
    let desk_list_view = desk_list
        .iter()
        .map(|x| match config.local_name {
            Some(_) => PotentialDesk {
                name: x.name.to_string(),
                status: SavedDeskStates::Saved.as_str().to_string(),
            },
            None => PotentialDesk {
                name: x.name.to_string(),
                status: SavedDeskStates::New.as_str().to_string(),
            },
        })
        .collect::<Vec<PotentialDesk>>();

    println!("Found desk list: {:?}", &desk_list_view);

    Ok(desk_list_view)
}

pub async fn connect_to_desk_by_name_internal(
    name: String,
    desk: &TauriSharedDesk,
) -> Result<PlatformPeripheral, ()> {
    let desk_to_connect = get_list_of_desks(&Some(name.clone())).await;
    let desk_to_connect = desk_to_connect
        .into_iter()
        .next()
        .expect("Error while getting a desk to connect to");
    let desk_to_connect = desk_to_connect.perp;
    println!("after desk to connect!");

    config_utils::save_local_name(name);
    println!("saved desk!");
    setup_bt_desk_device(&desk_to_connect).await;

    Ok(desk_to_connect)
}
