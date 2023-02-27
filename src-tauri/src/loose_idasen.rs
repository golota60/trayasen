use std::{
    cmp::Ordering,
    thread::sleep,
    time::{Duration, Instant},
};

use btleplug::api::{
    BDAddr, Characteristic, ParseBDAddrError, Peripheral as ApiPeripheral, WriteType,
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

/// convert desk response from bytes to tenth of millimeters and a speed of unknown dimension
///
/// ```
/// assert_eq!(idasen::bytes_to_position_speed(&[0x64, 0x19, 0x00, 0x00]), idasen::PositionSpeed{ position: idasen::MAX_HEIGHT, speed: 0 });
/// assert_eq!(idasen::bytes_to_position_speed(&[0x00, 0x00, 0x00, 0x00]), idasen::PositionSpeed{ position: idasen::MIN_HEIGHT, speed: 0 });
/// assert_eq!(idasen::bytes_to_position_speed(&[0x51, 0x04, 0x00, 0x00]), idasen::PositionSpeed{ position: 7305, speed: 0 });
/// assert_eq!(idasen::bytes_to_position_speed(&[0x08, 0x08, 0x00, 0x00]), idasen::PositionSpeed{ position: 8256, speed: 0 });
/// assert_eq!(idasen::bytes_to_position_speed(&[0x64, 0x18, 0x00, 0x00]), idasen::PositionSpeed{ position: 12444, speed: 0 });
/// ```
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

pub struct Bruhdasen<T>
where
    T: ApiPeripheral,
{
    pub mac_addr: BDAddr,
    pub desk: T,
    pub control_characteristic: Characteristic,
    pub position_characteristic: Characteristic,
}

/// Instantiate the struct. Requires `Device` instance. Same as Idasen::new
pub async fn setup(desk: &impl ApiPeripheral) -> Result<Bruhdasen<impl ApiPeripheral>, Error> {
    let mac_addr = desk.address();
    desk.connect().await?;
    desk.discover_services().await?;

    let control_characteristic = desk
        .characteristics()
        .iter()
        .find(|c| c.uuid == CONTROL_UUID)
        .ok_or_else(|| Error::CharacteristicsNotFound("Control".to_string()))?
        .clone();

    let position_characteristic = desk
        .characteristics()
        .iter()
        .find(|c| c.uuid == POSITION_UUID)
        .ok_or_else(|| Error::CharacteristicsNotFound("Position".to_string()))?
        .clone();

    if desk.subscribe(&position_characteristic).await.is_err() {
        return Err(Error::CannotSubscribePosition);
    };

    Ok(Bruhdasen {
        desk: desk.to_owned(),
        mac_addr,
        control_characteristic,
        position_characteristic,
    })
}

/// Some would say that getting characteristics every time is wasteful - we're doing it anyways
/// TODO: Try to refactor this - maybe chuck this into shared tauri state?
pub async fn get_control_characteristic(desk: &impl ApiPeripheral) -> Characteristic {
    desk.characteristics()
        .iter()
        .find(|c| c.uuid == CONTROL_UUID)
        .ok_or_else(|| Error::CharacteristicsNotFound("Control".to_string()))
        .expect("err while getting characteristic")
        .clone()
}
/// Some would say that getting characteristics every time is wasteful - we're doing it anyways
/// TODO: Try to refactor this - maybe chuck this into shared tauri state?
pub async fn get_position_characteristic(desk: &impl ApiPeripheral) -> Characteristic {
    desk.characteristics()
        .iter()
        .find(|c| c.uuid == POSITION_UUID)
        .ok_or_else(|| Error::CharacteristicsNotFound("Position".to_string()))
        .expect("Error while getting position characteristic")
        .clone()
}

/// Move desk up.
pub async fn up(desk: &impl ApiPeripheral) -> btleplug::Result<()> {
    let control_characteristic = get_control_characteristic(desk).await;

    desk.write(&control_characteristic, &UP, WriteType::WithoutResponse)
        .await
}

/// Lower the desk's position.
pub async fn down(desk: &impl ApiPeripheral) -> btleplug::Result<()> {
    let control_characteristic = get_control_characteristic(desk).await;
    desk.write(&control_characteristic, &DOWN, WriteType::WithoutResponse)
        .await
}

/// Stop desk from moving.
pub async fn stop(desk: &impl ApiPeripheral) -> btleplug::Result<()> {
    let control_characteristic = get_control_characteristic(desk).await;
    desk.write(&control_characteristic, &STOP, WriteType::WithoutResponse)
        .await
}

/// Move desk to a desired position. The precision is decent, usually less than 1mm off.
pub async fn move_to(desk: &impl ApiPeripheral, target_position: u16) -> Result<(), Error> {
    move_to_target(desk, target_position).await
}

async fn move_to_target(desk: &impl ApiPeripheral, target_position: u16) -> Result<(), Error> {
    println!("starting moving to target");
    if !(MIN_HEIGHT..=MAX_HEIGHT).contains(&target_position) {
        return Err(Error::PositionNotInRange);
    }

    let mut position_reached = false;
    let last_position = position(desk).await? as i16;
    let last_position_read_at = Instant::now();
    let target_position = target_position as i16;
    while !position_reached {
        sleep(Duration::from_millis(200));
        let current_position = position(desk).await? as i16;
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

/// Return the desk height in tenth millimeters (1m = 10000)
pub async fn position(desk: &impl ApiPeripheral) -> Result<u16, Error> {
    Ok(position_and_speed(desk).await?.position)
}

/// Return the denk height in tenth millimeters and speed in unknown dimension
pub async fn position_and_speed(desk: &impl ApiPeripheral) -> Result<PositionSpeed, Error> {
    let position_characteristic = get_position_characteristic(desk).await;

    let value = desk.read(&position_characteristic).await?;
    Ok(bytes_to_position_speed(&value))
}
