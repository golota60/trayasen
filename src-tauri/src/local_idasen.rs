use btleplug::api::{BDAddr, Peripheral, PeripheralProperties};

use crate::broken_idasen::{get_desks, Device, Error, Idasen};

// #[tauri::command]
// pub async fn get_test() -> Vec<PeripheralProperties> {
//     let desks = get_desks(None).await.expect("asd");

//     let mut x: Vec<String> = vec![];

//     for y in desks {
//         let z = y.properties().await.expect("error:lol").unwrap();

//         x.push(z.to_string());
//     }
// }

pub async fn get_list_of_desks(mac: &Option<String>) -> Vec<impl Peripheral> {
    let desks = match mac {
        // If MAC was provided
        Some(mac_value) => {
            let addr = mac_value.as_str().parse::<BDAddr>();

            match addr {
                Ok(addr) => {
                    let desks = get_desks(Some(addr)).await;
                    desks
                }
                Err(err) => Err(Error::MacAddrParseFailed(err)),
            }
        }
        // If MAC was NOT provided
        None => {
            let desks = get_desks(None).await;
            desks
        }
    };
    let desks = desks.expect("Error while getting a list of desks");

    desks
}

/// Get the desk instance. MAC Address is optional - if provided, it will be used.
pub async fn get_universal_instance(mac: &Option<String>) -> Result<Idasen<impl Device>, Error> {
    let desk = match mac {
        // If MAC was provided
        Some(mac_value) => {
            let addr = mac_value.as_str().parse::<BDAddr>();

            match addr {
                Ok(addr) => {
                    let desks = get_desks(Some(addr)).await?;

                    Idasen::new(desks.into_iter().next().ok_or(Error::CannotFindDevice)?).await
                }
                Err(err) => Err(Error::MacAddrParseFailed(err)),
            }
        }
        // If MAC was NOT provided
        None => {
            let desks = get_desks(None).await?;
            let desk = Idasen::new(desks.into_iter().next().ok_or(Error::CannotFindDevice)?).await;
            desk
        }
    };
    desk
}
