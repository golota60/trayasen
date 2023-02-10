pub use btleplug::api::Peripheral as Device;
use btleplug::api::{
    BDAddr, Central, Characteristic, Manager as _, ParseBDAddrError, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use indicatif::ProgressBar;
use std::thread::sleep;
use std::time::Duration;
use std::{
    cmp::{max, Ordering},
    time::Instant,
};
// use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

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

    // shits unused
    // #[error("Cannot connect to the device.")]
    // Connectionerrored,

    // #[error("Cannot scan for devices.")]
    // Scanerrored,

    // #[error("Permission denied.")]
    // PermissionDenied,

    // #[error("Cannot discover Bluetooth characteristics.")]
    // CharacteristicsDiscoveryFailed,
    #[error("Bluetooth characteristics not found: '{}'.", _0)]
    CharacteristicsNotFound(String),

    #[error("Desired position has to be between MIN_HEIGHT and MAX_HEIGHT.")]
    PositionNotInRange,

    #[error("Cannot subscribe to read position.")]
    CannotSubscribePosition,

    // #[error("Cannot read position.")]
    // CannotReadPosition,
    #[error("errored to parse mac address.")]
    MacAddrParseFailed(#[from] ParseBDAddrError),

    #[error("bluetooth error {0}")]
    BtlePlugError(#[from] btleplug::Error),
}

pub struct ExpandedDesk {
    pub perp: Peripheral,
    pub name: String,
}

pub async fn get_desks(loc_name: Option<String>) -> Result<Vec<ExpandedDesk>, Error> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let mut jobs = Vec::new();
    // let loc_clonse =

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
) -> Result<Vec<ExpandedDesk>, Error> {
    adapter.start_scan(ScanFilter::default()).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    let mut desks = Vec::new();
    for peripheral in adapter.peripherals().await? {
        if let Some(props) = peripheral.properties().await? {
            if match name {
                Some(ref device_name) => {
                    // WE MATCHING BY NAME IN THIS MF CAUSE MACOS DOESNT GIVE US MAC ADDRESS - IM ON MY FUCK MACOS ARC
                    let y = props.address;
                    println!("y: {}", y);

                    // some devices might not have a local name
                    let name = props.local_name.clone().unwrap_or("".to_string());

                    device_name == &name
                }
                None => props.local_name.iter().any(|name| name.contains("Desk")),
            } {
                desks.push(ExpandedDesk {
                    perp: peripheral,
                    name: props.local_name.unwrap_or("".to_string()),
                }); //ere
            }
        }
    }
    Ok(desks)
}

// shits unused
/// Get instance of `Idasen` struct. The desk will be discovered by the name. If multiple are
/// applicable a random one will be choosen.
// pub async fn get_instance() -> Result<Idasen<impl Device>, Error> {
//     let desks = get_desks(None).await?;
//     Idasen::new(
//         desks
//             .into_iter()
//             .next().ok_or(Error::CannotFindDevice)?,
//     )
//     .await
// }

// /// Get the desk instance by it's Bluetooth MAC address (BD_ADDR).
// /// The address can be obtained also by accessing `mac_addr` property
// /// on instantiated `Idasen` instance.
// pub async fn get_instance_by_mac(mac: &str) -> Result<Idasen<impl Device>, Error> {
//     let addr = mac.parse::<BDAddr>();
//     match addr {
//         Ok(addr) => {
//             let desks = get_desks(Some(addr)).await?;
//             Ok(Idasen::new(
//                 desks
//                     .into_iter()
//                     .next().ok_or(Error::CannotFindDevice)?,
//             )
//             .await?)
//         }
//         Err(err) => Err(Error::MacAddrParseFailed(err)),
//     }
// }

pub struct Idasen<T>
where
    T: Device,
{
    pub mac_addr: BDAddr,
    pub desk: T,
    control_characteristic: Characteristic,
    position_characteristic: Characteristic,
}

impl<T: Device> Idasen<T> {
    /// Instantiate the struct. Requires `Device` instance.
    pub async fn new(desk: T) -> Result<Self, Error> {
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

        Ok(Self {
            desk,
            mac_addr,
            control_characteristic,
            position_characteristic,
        })
    }

    /// Move desk up.
    pub async fn up(&self) -> btleplug::Result<()> {
        self.desk
            .write(
                &self.control_characteristic,
                &UP,
                WriteType::WithoutResponse,
            )
            .await
    }

    /// Lower the desk's position.
    pub async fn down(&self) -> btleplug::Result<()> {
        self.desk
            .write(
                &self.control_characteristic,
                &DOWN,
                WriteType::WithoutResponse,
            )
            .await
    }

    /// Stop desk from moving.
    pub async fn stop(&self) -> btleplug::Result<()> {
        self.desk
            .write(
                &self.control_characteristic,
                &STOP,
                WriteType::WithoutResponse,
            )
            .await
    }

    /// Move desk to a desired position. The precision is decent, usually less than 1mm off.
    pub async fn move_to(&self, target_position: u16) -> Result<(), Error> {
        self.move_to_target(target_position, None).await
    }

    // shits unused
    // pub async fn move_to_with_progress(&self, target_position: u16) -> Result<(), Error> {
    //     let initial_position = (target_position as i16 - self.position().await? as i16).abs();
    //     let progress = ProgressBar::new(initial_position as u64);
    //     progress.set_style(ProgressStyle::default_bar().template("{spinner} {wide_bar} [{msg}cm]"));
    //     self.move_to_target(target_position, Some(progress)).await
    // }

    async fn move_to_target(
        &self,
        target_position: u16,
        progress: Option<ProgressBar>,
    ) -> Result<(), Error> {
        println!("starting moving to target");
        if !(MIN_HEIGHT..=MAX_HEIGHT).contains(&target_position) {
            return Err(Error::PositionNotInRange);
        }

        let mut position_reached = false;
        let mut last_position = self.position().await? as i16;
        let mut last_position_read_at = Instant::now();
        let target_position = target_position as i16;
        while !position_reached {
            sleep(Duration::from_millis(200));
            let current_position = self.position().await? as i16;
            let going_up = match target_position.cmp(&current_position) {
                Ordering::Greater => true,
                Ordering::Less => false,
                Ordering::Equal => return Ok(()),
            };
            let remaining_distance = (target_position - current_position).abs();
            // let elapsed_millis = last_position_read_at.elapsed().as_millis();
            // let moved_height = (last_position - current_position).abs();

            println!("lastpos: {}, lastposreadat: {:?}, rem_dist: {}", last_position, last_position_read_at, remaining_distance);

            // Tenth of millimetres per second
            // let speed = ((moved_height as f64 / elapsed_millis as f64) * 1000f64) as i16;

            // if let Some(ref progress) = progress {
            //     progress.inc(speed as u64);
            //     let position_cm = current_position as f32 / 100.0;
            //     progress.set_message(format!("{}", position_cm));
            // }

            // If under/over 1cm we call it a day. From my testing it's under <3mm always(sometimes it might fuck up and do like 8mm but fuck it)
            if remaining_distance <= 100 {
                println!("position reached!");
                position_reached = true;
                self.stop().await?;
            } else if going_up {
                self.up().await?;
            } else if !going_up {
                self.down().await?;
            }

            // If we're either:
            // * less than 5 millimetres, or:
            // * less than half a second from target
            // then we need to stop every iteration so that we don't overshoot
            // if remaining_distance < max(speed / 2, 50) {
            //     self.stop().await?;
            // }

            // Read last_position again to avoid weird speed readings when switching direction
            // last_position = self.position().await? as i16;
            // last_position_read_at = Instant::now();
        }

        // if let Some(progress) = progress {
        //     progress.finish();
        // }

        Ok(())
    }

    /// Return the desk height in tenth millimeters (1m = 10000)
    pub async fn position(&self) -> Result<u16, Error> {
        Ok(self.position_and_speed().await?.position)
    }

    /// Return the denk height in tenth millimeters and speed in unknown dimension
    pub async fn position_and_speed(&self) -> Result<PositionSpeed, Error> {
        let value = self.desk.read(&self.position_characteristic).await?;
        Ok(bytes_to_position_speed(&value))
    }

    // shits unused
    // Listen to position and speed changes
    // pub async fn position_and_speed_stream(
    //     &self,
    // ) -> Result<impl Stream<Item = PositionSpeed>, Error> {
    //     Ok(self.desk.notifications().await?.filter_map(|notification| {
    //         if notification.uuid == POSITION_UUID {
    //             Some(bytes_to_position_speed(&notification.value))
    //         } else {
    //             None
    //         }
    //     }))
    // }
}
