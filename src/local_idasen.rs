use btleplug::api::BDAddr;
use idasen::{get_desks, Device, Error, Idasen};

/// Get the desk instance. MAC Address is optional - if provided, it will be used.
pub async fn get_universal_instance(mac: &Option<String>) -> Result<Idasen<impl Device>, Error> {
    let desk = match mac {
        // If MAC was provided
        Some(mac_value) => {
            let addr = mac_value.as_str().parse::<BDAddr>();

            match addr {
                Ok(addr) => {
                    let desks = get_desks(Some(addr)).await?;
                    Ok(
                        Idasen::new(desks.into_iter().next().ok_or(Error::CannotFindDevice)?)
                            .await?,
                    )
                }
                Err(err) => Err(Error::MacAddrParseFailed(err)),
            }
        }
        // If MAC was NOT provided
        None => {
            let desks = get_desks(None).await?;
            Idasen::new(desks.into_iter().next().ok_or(Error::CannotFindDevice)?).await
        }
    };
    desk
}
