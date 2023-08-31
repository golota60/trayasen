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
use uuid::Uuid;

/*
  This file contains loose utils to interact with the peripheral as if it's a desk.
  Created it cause having an instance of a Idasen desk on top of a perhiperal poses a lot of problems
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
pub enum Error {
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

pub struct Idasen<T>
where
    T: ApiPeripheral,
{
    pub mac_addr: BDAddr,
    pub desk: T,
    pub control_characteristic: Characteristic,
    pub position_characteristic: Characteristic,
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

/// Do a set of tasks for a peripheral to make it usable.
pub async fn setup(desk: &impl ApiPeripheral) -> Result<Idasen<impl ApiPeripheral>, Error> {
    let mac_addr = BDAddr::default(); //desk.address();
    println!("got the mac! desk: {:?}", &desk);
    let x = desk.connect().await.unwrap();
    println!("connected!!");
    let y = desk.discover_services().await.unwrap();
    println!("discovered!");

    let control_characteristic = desk
        .characteristics()
        .iter()
        .find(|c| c.uuid == CONTROL_UUID)
        .ok_or_else(|| Error::CharacteristicsNotFound("Control".to_string()))?
        .clone();
    println!("got the characteristics!");

    let position_characteristic = desk
        .characteristics()
        .iter()
        .find(|c| c.uuid == POSITION_UUID)
        .ok_or_else(|| Error::CharacteristicsNotFound("Position".to_string()))?
        .clone();
    println!("got the position characteristics!!");

    if desk.subscribe(&position_characteristic).await.is_err() {
        return Err(Error::CannotSubscribePosition);
    };
    println!("subscribed!!");

    Ok(Idasen {
        desk: desk.to_owned(),
        mac_addr,
        control_characteristic,
        position_characteristic,
    })
}

// Getting characteristics every time is wasteful
// TODO: Try to refactor this - maybe chuck this into shared tauri state?
pub async fn get_control_characteristic(desk: &impl ApiPeripheral) -> Characteristic {
    desk.characteristics()
        .iter()
        .find(|c| c.uuid == CONTROL_UUID)
        .ok_or_else(|| Error::CharacteristicsNotFound("Control".to_string()))
        .expect("err while getting characteristic")
        .clone()
}

pub async fn get_position_characteristic(desk: &impl ApiPeripheral) -> Characteristic {
    desk.characteristics()
        .iter()
        .find(|c| c.uuid == POSITION_UUID)
        .ok_or_else(|| Error::CharacteristicsNotFound("Position".to_string()))
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

pub async fn move_to_target(desk: &impl ApiPeripheral, target_position: u16) -> Result<(), Error> {
    println!("starting moving to target");
    if !(MIN_HEIGHT..=MAX_HEIGHT).contains(&target_position) {
        return Err(Error::PositionNotInRange);
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

pub async fn get_position(desk: &impl ApiPeripheral) -> Result<u16, Error> {
    Ok(get_position_and_speed(desk).await?.position)
}

pub async fn get_position_and_speed(desk: &impl ApiPeripheral) -> Result<PositionSpeed, Error> {
    let position_characteristic = get_position_characteristic(desk).await;

    let value = desk.read(&position_characteristic).await?;
    Ok(bytes_to_position_speed(&value))
}

/// Peripheral expanded with it's name(we treat it as an ID)
pub struct ExpandedPeripheral {
    pub perp: PlatformPeripheral,
    pub name: String,
}

pub async fn get_desks(loc_name: Option<String>) -> Result<Vec<ExpandedPeripheral>, Error> {
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
        Err(Error::CannotFindDevice)
    } else {
        Ok(desks)
    }
}

async fn search_adapter_for_desks(
    adapter: Adapter,
    name: Option<String>,
) -> Result<Vec<ExpandedPeripheral>, Error> {
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
