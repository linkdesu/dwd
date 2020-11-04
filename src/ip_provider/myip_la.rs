use crate::util::get;

pub async fn get_ip() -> Result<String, String> {
    get("https://api.myip.la").await
}
