//! The is public IP provider module, take a look at myip_la.rs or myip_ipip_net.rs then your will
//! know how to add your own provider.
//! At last don't forget to add it to the `fn get_ip` below.

mod myip_ipip_net;
mod myip_la;

/// Get public IP from different provider.
pub async fn get_ip(provider: &str) -> Result<String, String> {
    match provider {
        "myip.la" => myip_la::get_ip().await,
        "myip.ipip.net" => myip_ipip_net::get_ip().await,
        _ => Err(format!("Provider {} does not supported.", provider)),
    }
}
