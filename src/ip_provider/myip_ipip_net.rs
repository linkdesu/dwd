use lazy_static::lazy_static;
use log::trace;
use regex::Regex;
use std::error::Error;

use super::super::util::get;

/// Get public IP from https://myip.ipip.net
///
/// The provider will response some text like "当前 IP：xxx.xxx.xxx.xxx  来自于：中国 XX XX  电信",
/// so I use regex to capture the IP in it.
///
/// # Example:
///
/// ```rust
/// let ip = myip_ipip_net::get_ip().await?;
/// ```
pub async fn get_ip() -> Result<String, Box<dyn Error>> {
    let response = get("https://myip.ipip.net").await?;

    lazy_static! {
        static ref RE: Regex = Regex::new(r"IP：((?:\d{1,3}\.){3}\d{1,3})").unwrap();
    }
    let ret = RE.captures(&response);
    if ret.is_none() {
        trace!("Response content: {}", &response.to_string().trim_end());
        return Err("Can not capture IP from response".into());
    }

    Ok(ret.unwrap().get(1).map_or("".into(), |m| m.as_str().into()))
}

#[cfg(test)]
mod tests {
    use super::super::super::util::is_ip;
    use super::*;

    #[tokio::test]
    async fn get_ip_should_works() {
        let ret = get_ip().await;
        assert!(ret.is_ok());
        assert!(is_ip(ret.as_ref().unwrap()));
    }
}
