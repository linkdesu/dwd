use log::trace;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::str;

use super::super::util::{error_style, info_style};

const BASE_URL: &str = "https://api.name.com/v4/domains/";

#[derive(Debug, Serialize, Deserialize)]
struct ApiError {
    message: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Record {
    id: i32,
    #[serde(rename(deserialize = "domainName", serialize = "domainName"))]
    domain_name: String,
    #[serde(rename(deserialize = "host", serialize = "host"))]
    record_host: Option<String>,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    record_type: String,
    #[serde(rename(deserialize = "answer", serialize = "answer"))]
    record_answer: String,
    #[serde(rename(deserialize = "ttl", serialize = "ttl"))]
    record_ttl: u32,
}

impl Record {
    fn update(&mut self, record_type: &str, record_answer: &str, record_ttl: u32) {
        self.record_type = String::from(record_type);
        self.record_answer = String::from(record_answer);
        self.record_ttl = record_ttl;
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RecordList {
    records: Vec<Record>,
    #[serde(rename(deserialize = "nextPage", serialize = "next_page"))]
    next_page: Option<i32>,
    #[serde(rename(deserialize = "lastPage", serialize = "last_page"))]
    last_page: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigNameCom {
    pub domain: String,
    pub username: Option<String>,
    pub token: Option<String>,
    pub record_type: String,
    pub record_host: Option<String>,
    pub record_ttl: u32,
}

/// Update DNS record on name.com
///
/// The API version is V4.
/// The document of name.com API: https://www.name.com/api-docs/
///
/// # Example:
/// ```rust
/// name_com::update(domain, record_type, record_host, ip, record_ttl).await?;
/// ```
pub async fn update(conf: &ConfigNameCom, ip: &str) -> Result<(), Box<dyn Error>> {
    let username = match conf.username.as_ref() {
        Some(val) => val.to_owned(),
        None => env::var("NAME_COM_USERNAME")
            .map_err(|_| "Please set username in config file or through env variable USERNAME.")?,
    };
    let token = match conf.token.as_ref() {
        Some(val) => val.to_owned(),
        None => env::var("NAME_COM_TOKEN").map_err(|_| "Please set env variable TOKEN.")?,
    };

    trace!("Username: {:?}", info_style(&username));
    trace!("Token: {:?}", info_style(&token));

    let client = Client::new();
    let base_url = Url::parse(BASE_URL).map_err(|e| e.to_string())?;

    let ret = find_record(&client, &base_url, &username, &token, &conf.domain, &conf.record_host).await?;
    if ret.is_none() {
        todo!()
    }

    let mut record = ret.unwrap();
    record.update(&conf.record_type, ip, conf.record_ttl);

    update_record(&client, &base_url, &username, &token, record).await?;

    Ok(())
}

/// Find DNS record by host
///
/// Because name.com do not support fetch record by host directly, so we need to find the record first.
///
/// > Pagination is not supported here.
async fn find_record(
    client: &Client,
    base_url: &Url,
    username: &str,
    token: &str,
    domain: &str,
    record_host: &Option<String>,
) -> Result<Option<Record>, Box<dyn Error>> {
    let url = base_url.join(&format!("{}/records", domain))?;
    let response = client.get(url).basic_auth(username, Some(token)).send().await?;

    trace!("GET {} {}", response.url(), response.status());

    if response.error_for_status_ref().is_err() {
        let api_error = response.json::<ApiError>().await?;
        return Err(format!("API response error: {}", error_style(api_error.message)).into());
    }

    let record_list = response.json::<RecordList>().await.map_err(|e| e.to_string())?;
    let record = record_list.records.into_iter().find(|record| {
        if record_host.is_none() && record.record_host.is_none() {
            return true;
        } else if record.record_host.is_some() && record_host == &record.record_host {
            return true;
        }

        false
    });

    trace!("Find record from name.com: {:?}", record);

    Ok(record)
}

/// Update DNS record
async fn update_record(
    client: &Client,
    base_url: &Url,
    username: &str,
    token: &str,
    record: Record,
) -> Result<(), Box<dyn Error>> {
    let url = base_url.join(&format!("{}/records/{}", record.domain_name, record.id))?;
    let request = client
        .put(url)
        .json(&record)
        .basic_auth(username, Some(token))
        .build()?;
    let raw_body = str::from_utf8(request.body().unwrap().as_bytes().unwrap())
        .unwrap()
        .to_owned();
    let response = client.execute(request).await?;

    trace!("PUT {} {}", response.url(), response.status());
    trace!("Body: {}", raw_body);

    if response.error_for_status_ref().is_err() {
        let api_error = response.json::<ApiError>().await?;
        return Err(format!("API response error: {}", error_style(api_error.message)).into());
    }

    trace!("Update record to: {:?}", record);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::super::util::is_ip;
    use super::*;
    use dotenv::dotenv;

    fn before() -> (Client, Url, String, String) {
        dotenv().ok();

        let client = Client::new();
        let url = Url::parse(BASE_URL)
            .map_err(|e| e.to_string())
            .expect("Expect valid base url.");
        let username = env::var("NAME_COM_USERNAME").expect("Please config .env for environments.");
        let password = env::var("NAME_COM_TOKEN").expect("Please config .env for environments.");

        (client, url, username, password)
    }

    #[tokio::test]
    async fn find_record_should_works() {
        let (client, url, username, password) = before();
        let record = Some(String::from("dwd-unittest"));
        let ret = find_record(&client, &url, &username, &password, "xieal.me", &record).await;
        assert!(ret.is_ok(), "{}", ret.unwrap_err().to_string());

        println!("ret = {:?}", ret);
        let record = ret.unwrap().unwrap();
        assert_eq!(record.domain_name, String::from("xieal.me"));
        assert_eq!(record.record_host, Some(String::from("dwd-unittest")));
        assert!(is_ip(&record.record_answer));
    }

    #[tokio::test]
    async fn update_record_should_works() {
        let (client, url, username, password) = before();
        let record = Record {
            id: 214943323, // WARNING, This is retrieved by find_record.
            domain_name: String::from("xieal.me"),
            record_host: Some(String::from("dwd-unittest")),
            record_type: String::from("A"),
            record_answer: String::from("127.0.0.1"),
            record_ttl: 300,
        };

        let ret = update_record(&client, &url, &username, &password, record).await;
        assert!(ret.is_ok(), "{}", ret.unwrap_err().to_string());
    }
}
