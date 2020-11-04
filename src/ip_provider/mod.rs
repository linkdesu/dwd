mod myip_la;
mod myip_ipip_net;

pub async fn get_ip (provider: &str) -> Result<String, String> {
    match provider {
        "myip.la" => myip_la::get_ip().await,
        "myip.ipip.net" => myip_ipip_net::get_ip().await,
        _ => Err(format!("Provider {} does not supported.", provider))
    }
}
