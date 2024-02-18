use anyhow::anyhow;
use reqwasm::http::Response;
use serde::{de::DeserializeOwned, Serialize};

pub async fn post<T>(url: &str, body: &T) -> Result<Response, reqwasm::Error>
where
    T: Serialize + ?Sized,
{
    let body = serde_json::to_value(body)?;
    reqwasm::http::Request::post(url)
        .body(body.to_string())
        .credentials(web_sys::RequestCredentials::Include)
        .header("content-type", "application/json")
        .send()
        .await
}

pub async fn get<T: DeserializeOwned>(url: &str) -> Result<Option<T>, anyhow::Error> {
    let response = reqwasm::http::Request::get(url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await?;

    if !response.ok() {
        return Err(anyhow!("Response is not ok"));
    }

    match response.json().await {
        Ok(json) => Ok(Some(json)),
        Err(_) => Ok(None),
    }
}
