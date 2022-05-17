use log::trace;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::str;

use super::super::util::{error_style, info_style};

const BASE_URL: &str = "https://dynv6.com/api/update";

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigDynv6Com {
    pub zone: String,
    pub token: Option<String>,
}

/// Update DDNS record on dynv6.com
///
/// The document of dynv6.com API: https://dynv6.com/docs/apis#rest
///
/// # Example:
/// ```rust
/// dynv6_com::update(domain, record_type, record_host, ip, record_ttl).await?;
/// ```
pub async fn update(conf: &ConfigDynv6Com, ip: &str) -> Result<(), Box<dyn Error>> {
    let token = match conf.token.as_ref() {
        Some(val) => val.to_owned(),
        None => env::var("DYNV6_COM_TOKEN").map_err(|_| "Please set env variable DYNV6_COM_TOKEN.")?,
    };

    trace!("Token: {:?}", info_style(&token));

    let client = Client::new();
    let base_url = Url::parse(BASE_URL).map_err(|e| e.to_string())?;

    let request = client
        .get(base_url)
        .query(&[("zone", conf.zone.as_str()), ("token", token.as_str()), ("ipv4", ip)])
        .build()?;
    let response = client.execute(request).await?;

    trace!("GET {} {}", response.url(), response.status());

    if let Err(err) = response.error_for_status_ref() {
        return Err(format!("API response error: {}", error_style(err)).into());
    }

    trace!("Update record to: {:?}", conf.zone);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn dynv6_update_should_works() {
        dotenv().ok();

        let ret = update("dwd-unittest.dynv6.net", "127.0.0.1").await;
        assert!(ret.is_ok(), "{}", ret.unwrap_err().to_string());
    }
}
