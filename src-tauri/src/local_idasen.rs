use crate::loose_idasen::{get_desks, ExpandedPeripheral};

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
