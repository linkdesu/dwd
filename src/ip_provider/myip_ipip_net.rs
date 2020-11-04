use regex::Regex;
use lazy_static::lazy_static;

use super::super::util::{get, v2, verbose_log};

pub async fn get_ip() -> Result<String, String> {
    let response = get("https://myip.ipip.net").await?;

    lazy_static! {
        static ref RE: Regex = Regex::new(r"IP：(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})").unwrap();
    }
    let ret = RE.captures(&response);
    if ret.is_none() {
        v2!("Response content: {}", &response);
        return Err(String::from("Can not capture IP from response"));
    }

    Ok(ret.unwrap().get(1).map_or("".into(), |m| m.as_str().into()))
}
