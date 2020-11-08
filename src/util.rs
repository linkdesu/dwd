use console::{Style, StyledObject};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::IntoUrl;
use std::net::{Ipv4Addr, Ipv6Addr};

pub use vlog::{set_verbosity_level, v0, v1, v2, verbose_log};

lazy_static! {
    static ref ERROR: Style = Style::new().red();
    static ref WARN: Style = Style::new().yellow();
    static ref INFO: Style = Style::new().cyan();
    static ref SUCCESS: Style = Style::new().green();
}

lazy_static! {
    static ref IPV4_RE: Regex = Regex::new(r"^(?:\d{1,3}\.){3}\d{1,3}$").unwrap();
    static ref IPV6_RE: Regex = Regex::new(r"i^(?:[0-9a-f]{1,4}\:){7}[0-9a-f]{1,4}$").unwrap();
}

pub fn error<T>(content: T) -> StyledObject<T> {
    ERROR.apply_to(content)
}

pub fn warn<T>(content: T) -> StyledObject<T> {
    WARN.apply_to(content)
}

pub fn info<T>(content: T) -> StyledObject<T> {
    INFO.apply_to(content)
}

pub fn success<T>(content: T) -> StyledObject<T> {
    SUCCESS.apply_to(content)
}

pub async fn get<T: IntoUrl>(url: T) -> Result<String, String> {
    reqwest::get(url)
        .await
        .map_err(|err| err.to_string())?
        .text()
        .await
        .map_err(|err| err.to_string())
}

pub fn is_ip(ip: &str) -> bool {
    ip.parse::<Ipv4Addr>().is_ok() || ip.parse::<Ipv6Addr>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_ip_should_support_ipv4() {
        assert!(is_ip("1.1.1.1"));
        assert!(is_ip("255.255.255.255"));
        assert!(is_ip("127.0.0.1"));
    }

    #[test]
    fn is_ip_should_support_ipv6() {
        assert!(is_ip("2001:0DB8:02de:0000:0000:0000:0000:0e13"));
        assert!(is_ip("2001:DB8:2de:0:0:0:0:e13"));
        assert!(is_ip("2001:DB8:2de::e13"));
    }

    #[test]
    fn is_ip_should_failed_for_invalid_ip_string() {
        assert!(!is_ip("hello world"));
        assert!(!is_ip("127.0.0"));
        assert!(!is_ip("999.999.999.999"));
        assert!(!is_ip("2001::DB8:2de::e13"));
    }
}
