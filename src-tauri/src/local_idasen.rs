use btleplug::api::{BDAddr, Peripheral, PeripheralProperties};

use crate::broken_idasen::{self, get_desks, Device, Error, Idasen};

// #[tauri::command]
// pub async fn get_test() -> Vec<PeripheralProperties> {
//     let desks = get_desks(None).await.expect("asd");

//     let mut x: Vec<String> = vec![];

//     for y in desks {
//         let z = y.properties().await.expect("error:lol").unwrap();

//         x.push(z.to_string());
//     }
// }

pub async fn get_list_of_desks(loc_name: &Option<String>) -> Vec<broken_idasen::ExpandedDesk> {
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

/// Get the desk instance. MAC Address is optional - if provided, it will be used.
pub async fn get_universal_instance(
    loc_name: &Option<String>,
) -> Result<Idasen<impl Device>, Error> {
    let desk = match loc_name {
        // If MAC was provided
        Some(loc_name) => {
            let desks = get_desks(Some(loc_name.clone())).await?;

            Idasen::new(
                desks
                    .into_iter()
                    .next()
                    .ok_or(Error::CannotFindDevice)?
                    .perp,
            )
            .await
        }
        // If MAC was NOT provided
        None => {
            let desks = get_desks(None).await?;
            let desk = Idasen::new(
                desks
                    .into_iter()
                    .next()
                    .ok_or(Error::CannotFindDevice)?
                    .perp,
            )
            .await;
            desk
        }
    };
    desk
}
