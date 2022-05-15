use std::error::Error;

use crate::util::get;

/// Get public IP from https://api.myip.la
///
/// The homepage of the provider is https://www.myip.la.
/// When request https://api.myip.la it will return IPv4 or IPv6 base on your network status.
///
/// # Example:
///
/// ```rust
/// let ip = myip_la::get_ip().await?;
/// ```
pub async fn get_ip() -> Result<String, Box<dyn Error>> {
    get("https://api.myip.la").await.map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::super::super::util::is_ip;
    use super::*;

    #[tokio::test]
    async fn get_ip_should_works() {
        let ret = get_ip().await;
        println!("ret = {:?}", ret);
        assert!(ret.is_ok());
        assert!(is_ip(ret.as_ref().unwrap()));
    }
}
