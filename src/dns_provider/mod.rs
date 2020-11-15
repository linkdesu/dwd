//! This is DNS provider module, take a look at name_com.rs for how to add your own provider.
//! At last don't forget to add it to the `fn update_record` below.

use std::process;

use super::util::{cross, error, v0, verbose_log};

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
    let ret;

    match provider {
        "name.com" => {
            ret = name_com::update(domain, record_type, record_host, ip, record_ttl).await
        }
        // "aliyun" => todo!(),
        _ => {
            v0!(
                "{} Provider {} does not supported.",
                cross(),
                error(provider)
            );
            process::exit(1);
        }
    };

    if let Err(e) = ret {
        v0!("{} {}", cross(), e);
        return Err(());
    }

    Ok(())
}
