use btleplug::api::{BDAddr, Characteristic, ParseBDAddrError, Peripheral, WriteType};
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
    T: Peripheral,
{
    pub mac_addr: BDAddr,
    pub desk: T,
    pub control_characteristic: Characteristic,
    pub position_characteristic: Characteristic,
}

/// Instantiate the struct. Requires `Device` instance. Same as Idasen::new
pub async fn setup(desk: &impl Peripheral) -> Result<Bruhdasen<impl Peripheral>, Error> {
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

pub async fn get_post_chars(desk: &impl Peripheral) -> Characteristic {
    desk.characteristics()
        .iter()
        .find(|c| c.uuid == CONTROL_UUID)
        .ok_or_else(|| Error::CharacteristicsNotFound("Control".to_string()))
        .expect("err while getting characteristic")
        .clone()
}

/// Move desk up.
pub async fn up(desk: &impl Peripheral) -> btleplug::Result<()> {
    let control_characteristic = get_post_chars(desk).await;

    desk.write(&control_characteristic, &UP, WriteType::WithoutResponse)
        .await
}

// /// Lower the desk's position.
// pub async fn down(&self) -> btleplug::Result<()> {
//     self.desk
//         .write(
//             &self.control_characteristic,
//             &DOWN,
//             WriteType::WithoutResponse,
//         )
//         .await
// }

// /// Stop desk from moving.
// pub async fn stop(&self) -> btleplug::Result<()> {
//     self.desk
//         .write(
//             &self.control_characteristic,
//             &STOP,
//             WriteType::WithoutResponse,
//         )
//         .await
// }

// /// Move desk to a desired position. The precision is decent, usually less than 1mm off.
// pub async fn move_to(&self, target_position: u16) -> Result<(), Error> {
//     self.move_to_target(target_position, None).await
// }

// async fn move_to_target(
//     &self,
//     target_position: u16,
//     progress: Option<ProgressBar>,
// ) -> Result<(), Error> {
//     println!("starting moving to target");
//     if !(MIN_HEIGHT..=MAX_HEIGHT).contains(&target_position) {
//         return Err(Error::PositionNotInRange);
//     }

//     let mut position_reached = false;
//     let last_position = self.position().await? as i16;
//     let last_position_read_at = Instant::now();
//     let target_position = target_position as i16;
//     while !position_reached {
//         sleep(Duration::from_millis(200));
//         let current_position = self.position().await? as i16;
//         let going_up = match target_position.cmp(&current_position) {
//             Ordering::Greater => true,
//             Ordering::Less => false,
//             Ordering::Equal => return Ok(()),
//         };
//         let remaining_distance = (target_position - current_position).abs();

//         println!(
//             "lastpos: {}, lastposreadat: {:?}, rem_dist: {}",
//             last_position, last_position_read_at, remaining_distance
//         );

//         // If under/over 1cm we call it a day. From my testing it's under <3mm always(sometimes it might fuck up and do like 8mm but fuck it)
//         if remaining_distance <= 100 {
//             println!("position reached!");
//             position_reached = true;
//             self.stop().await?;
//         } else if going_up {
//             self.up().await?;
//         } else if !going_up {
//             self.down().await?;
//         }
//     }

//     Ok(())
// }

// /// Return the desk height in tenth millimeters (1m = 10000)
// pub async fn position(&self) -> Result<u16, Error> {
//     Ok(self.position_and_speed().await?.position)
// }

// /// Return the denk height in tenth millimeters and speed in unknown dimension
// pub async fn position_and_speed(&self) -> Result<PositionSpeed, Error> {
//     let value = self.desk.read(&self.position_characteristic).await?;
//     Ok(bytes_to_position_speed(&value))
// }
