//! This is public IP provider module, take a look at myip_la.rs or myip_ipip_net.rs then your will
//! know how to add your own provider.
//! At last don't forget to add it to the `fn get_ip` below.
use log::{debug, error};
use simple_error::SimpleError;
use std::convert::TryFrom;

use super::util::{error_style, info_style};

mod myip_ipip_net;
mod myip_la;
mod vnet_one;

#[derive(Debug)]
pub enum IpProvider {
    IpipNet,
    MyipLa,
    VnetOne,
}

impl TryFrom<&str> for IpProvider {
    type Error = SimpleError;

    fn try_from(input: &str) -> Result<IpProvider, SimpleError> {
        let provider = match input {
            "ipip.net" => IpProvider::IpipNet,
            "myip.la" => IpProvider::MyipLa,
            "vnet.one" => IpProvider::VnetOne,
            _ => return Err(SimpleError::new("Unknown provider")),
        };
        Ok(provider)
    }
}

/// Get public IP from different provider.
pub async fn get_ip_by_fallback(providers: &[String]) -> Option<(String, String)> {
    debug!("Requesting {} for public IP ...", info_style(providers.join(", ")));

    let mut ret = None;
    for name in providers.iter() {
        let provider = match IpProvider::try_from(name.as_str()) {
            Ok(val) => val,
            Err(_) => {
                error!(target: "error", "IP provider {} does not supported.", error_style(name));
                continue;
            }
        };

        match get_ip(provider).await {
            Err(_) => continue,
            Ok(ip) => {
                ret = Some((name.to_owned(), ip));
                break;
            }
        }
    }

    ret
}

async fn get_ip(provider: IpProvider) -> Result<String, ()> {
    let ret = match provider {
        IpProvider::IpipNet => myip_ipip_net::get_ip().await,
        IpProvider::MyipLa => myip_la::get_ip().await,
        IpProvider::VnetOne => vnet_one::get_ip().await,
    };

    match ret {
        Err(err) => {
            error!(target: "error", "{}", err);
            Err(())
        }
        Ok(ip) => Ok(ip),
    }
}
