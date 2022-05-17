//! This is DNS provider module, take a look at name_com.rs for how to add your own provider.
//! At last don't forget to add it to the `fn update_record` below.

use lazy_static::lazy_static;
use log::{debug, error, info};
use simple_error::SimpleError;
use std::convert::TryFrom;

use super::config::Config;
use super::util::{error_style, info_style};

pub mod dynv6_com;
pub mod name_com;

lazy_static! {
    pub static ref DNS_PROVIDERS: Vec<&'static str> = vec!["name.com", "dynv6.com"];
}

#[derive(Debug)]
pub enum DnsProvider {
    NameCom,
    Dynv6Com,
}

impl TryFrom<&str> for DnsProvider {
    type Error = SimpleError;

    fn try_from(input: &str) -> Result<DnsProvider, SimpleError> {
        let provider = match input {
            "name.com" => DnsProvider::NameCom,
            "dynv6.com" => DnsProvider::Dynv6Com,
            _ => return Err(SimpleError::new("Unknown provider")),
        };
        Ok(provider)
    }
}

/// Update record through DNS provider API
///
/// ⚠️ This function suppose to be never crash!
pub async fn update_dns_for_all(conf: &Config, ip: &str) {
    let providers = &conf.dns_provider;
    debug!(
        "Requesting {} to update DNS record ...",
        info_style(providers.join(", "))
    );

    // TODO Replace this with somethind like Promise.all
    for name in providers.iter() {
        let provider = match DnsProvider::try_from(name.as_str()) {
            Ok(val) => val,
            Err(_) => {
                error!(target: "error", "DNS provider {} does not supported.", error_style(name));
                continue;
            }
        };

        update_record(conf, provider, ip).await;
    }
}

async fn update_record(conf: &Config, provider: DnsProvider, ip: &str) {
    let ret = match provider {
        DnsProvider::NameCom => match conf.name_com.as_ref() {
            Some(sub_conf) => name_com::update(sub_conf, ip).await,
            None => Err("The config.name_com is required.".into()),
        },
        DnsProvider::Dynv6Com => match conf.dynv6_com.as_ref() {
            Some(sub_conf) => dynv6_com::update(sub_conf, ip).await,
            None => Err("The config.dynv6_com is required.".into()),
        },
    };

    if let Err(e) = ret {
        error!(target: "error", "Update DNS provider {:?} failed, error: {}", provider, e);
    } else {
        info!( target: "success", "Successfully updated DNS provider {:?}.", provider );
    }
}
