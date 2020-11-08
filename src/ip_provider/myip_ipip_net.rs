use lazy_static::lazy_static;
use regex::Regex;

use super::super::util::{get, v2, verbose_log};

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
pub async fn get_ip() -> Result<String, String> {
    let response = get("https://myip.ipip.net").await?;

    lazy_static! {
        static ref RE: Regex = Regex::new(r"IP：((?:\d{1,3}\.){3}\d{1,3})").unwrap();
    }
    let ret = RE.captures(&response);
    if ret.is_none() {
        v2!("Response content: {}", &response);
        return Err(String::from("Can not capture IP from response"));
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
