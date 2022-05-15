use std::error::Error;

use crate::util::get;

/// Get public IP from https://ip.vnet.one/check.php
///
/// The homepage of the provider is https://www.vnet.one/ .
///
/// # Example:
///
/// ```rust
/// let ip = vnet_one::get_ip().await?;
/// ```
pub async fn get_ip() -> Result<String, Box<dyn Error>> {
    get("https://ip.vnet.one/check.php").await.map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::is_ip;

    #[tokio::test]
    async fn get_ip_should_works() {
        let ret = get_ip().await;
        println!("ret = {:?}", ret);
        assert!(ret.is_ok());
        assert!(is_ip(ret.as_ref().unwrap()));
    }
}
