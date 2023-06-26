use anyhow::anyhow;
use reqwasm::http::Response;
use serde::{de::DeserializeOwned, Serialize};

const BASE_URL: &str = "http://localhost:42069/";

pub async fn post<T>(url: &str, body: &T) -> Result<Response, reqwasm::Error>
where
    T: Serialize + ?Sized,
{
    let body = serde_json::to_value(body)?;
    reqwasm::http::Request::post(&format!("{}{}", BASE_URL, url))
        .body(body.to_string())
        // .credentials(web_sys::RequestCredentials::Include)
        .header("content-type", "application/json")
        .send()
        .await
}

pub async fn get(url: &str) -> Result<(), anyhow::Error> {
    let response = reqwasm::http::Request::get(&format!("{}{}", BASE_URL, url))
        // .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await?;

    Ok(())

    // if !response.ok() {
    //     return Err(anyhow!("Response is not ok"));
    // }

    // response
    //     .json()
    //     .await
    //     .map_err(|_| anyhow!("Couldn't convert to json"))
}
