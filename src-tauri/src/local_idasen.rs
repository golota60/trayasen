use btleplug::api::{BDAddr, Peripheral, PeripheralProperties};

use crate::broken_idasen::{self, get_desks, Device, Error, Idasen};

pub async fn get_list_of_desks(loc_name: &Option<String>) -> Vec<broken_idasen::ExpandedPeripheral> {
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

pub struct PowerIdasen<T>
where
    T: Peripheral,
{
    pub actual_idasen: Idasen<T>,
    pub local_name: String,
}

/// Get the desk instance. Local name is optional - if provided, it will be used.
pub async fn get_universal_instance(
    loc_name: &Option<String>,
) -> Result<PowerIdasen<impl Device>, Error> {
    let desk = match loc_name {
        // If MAC was provided
        Some(loc_name) => {
            let desks = get_desks(Some(loc_name.clone())).await?;

            let unwrap_desk = desks
                .into_iter()
                .next()
                .expect("error while unwrapping name");

            let idas = Idasen::new(unwrap_desk.perp)
                .await
                .expect("error while unwrapping the idasen");

            PowerIdasen {
                actual_idasen: idas,
                local_name: unwrap_desk.name,
            }
        }
        // If MAC was NOT provided
        None => {
            let desks = get_desks(None).await?;

            let unwrap_desk = desks
                .into_iter()
                .next()
                .expect("error while unwrapping name");

            let desk = Idasen::new(unwrap_desk.perp).await;
            PowerIdasen {
                actual_idasen: desk.expect("error while unwrapping the idasen w/o"),
                local_name: unwrap_desk.name,
            }
        }
    };

    Ok(desk)
}
