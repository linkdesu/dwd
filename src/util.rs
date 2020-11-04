use console::{Style, StyledObject};
use reqwest::IntoUrl;
use lazy_static::lazy_static;

pub use vlog::{set_verbosity_level, v0, v1, v2, verbose_log};

lazy_static! {
    static ref ERROR: Style = Style::new().red();
    static ref WARN: Style = Style::new().yellow();
    static ref INFO: Style = Style::new().cyan();
    static ref SUCCESS: Style = Style::new().green();
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
