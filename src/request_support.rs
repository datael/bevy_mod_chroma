use bevy::log::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug)]
pub enum RequestError {
    RequestFailed(reqwest::Error),
    ParseFailed(serde_json::error::Error),
}

impl From<reqwest::Error> for RequestError {
    fn from(error: reqwest::Error) -> Self {
        Self::RequestFailed(error)
    }
}

impl From<serde_json::error::Error> for RequestError {
    fn from(error: serde_json::error::Error) -> Self {
        Self::ParseFailed(error)
    }
}

#[derive(Debug, Deserialize)]
pub struct IgnoreContent {}

pub(crate) async fn post<ResponseBody: DeserializeOwned>(
    url: String,
    body: impl Serialize,
) -> Result<ResponseBody, RequestError> {
    let response = reqwest::Client::new().post(url).json(&body).send().await;

    if let Err(error) = &response {
        error!("post failed: {:?}", error);
    } else {
        info!("post succeeded");
    }

    let response_text = response?.text().await?;

    Ok(serde_json::from_str(&response_text)?)
}

pub(crate) async fn put<ResponseBody: DeserializeOwned>(
    url: String,
    body: impl Serialize,
) -> Result<ResponseBody, RequestError> {
    let response = reqwest::Client::new().put(url).json(&body).send().await;

    if let Err(error) = &response {
        error!("put failed: {:?}", error);
    } else {
        info!("put succeeded");
    }

    let response_text = response?.text().await?;

    Ok(serde_json::from_str(&response_text)?)
}
