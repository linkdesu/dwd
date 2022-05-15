//! This is public IP provider module, take a look at myip_la.rs or myip_ipip_net.rs then your will
//! know how to add your own provider.
//! At last don't forget to add it to the `fn get_ip` below.
use log::{debug, error};
use std::process;

use super::util::{error_style, info_style};

mod myip_ipip_net;
mod myip_la;
mod vnet_one;

/// Get public IP from different provider.
pub async fn get_ip(provider: &str) -> Result<String, ()> {
    debug!("Requesting {} for public IP ...", info_style(provider));

    let ret = match provider {
        "myip.la" => myip_la::get_ip().await,
        "myip.ipip.net" => myip_ipip_net::get_ip().await,
        "vnet.one" => vnet_one::get_ip().await,
        _ => {
            error!(target: "error", "Provider {} does not supported.", error_style(provider));
            process::exit(1);
        }
    };

    if let Err(e) = ret {
        error!(target: "error", "{}", e);
        return Err(());
    }

    Ok(ret.unwrap())
}
