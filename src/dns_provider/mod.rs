//! This is DNS provider module, take a look at name_com.rs for how to add your own provider.
//! At last don't forget to add it to the `fn update_record` below.

use log::{debug, error};
use std::process;

use super::util::{error_style, info_style};

mod name_com;

/// Update record through DNS provider API
///
/// ⚠️ This function suppose to be never crash!
pub async fn update_record(
    provider: &str,
    domain: &str,
    record_type: &str,
    record_host: Option<&str>,
    ip: &str,
    record_ttl: &u32,
) -> Result<(), ()> {
    debug!("Requesting {} to update DNS record ...", info_style(provider));

    let ret = match provider {
        "name.com" => name_com::update(domain, record_type, record_host, ip, record_ttl).await,
        // "aliyun" => todo!(),
        _ => {
            error!(target: "error", "Provider {} does not supported.", error_style(provider));
            process::exit(1);
        }
    };

    if let Err(e) = ret {
        error!(target: "error", "{}", e);
        return Err(());
    }

    Ok(())
}
