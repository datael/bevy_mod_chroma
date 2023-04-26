use crossbeam_channel::Receiver; // Cannot use mpsc because mpsc::Receiver is not Sync

use bevy::prelude::Component;
use reqwest::Url;
use serde::{de::DeserializeOwned, Serialize};

mod plugin;

pub trait Body: 'static + Send + Sync + Clone {}

pub trait RequestBody: Body + Serialize {}
pub trait ResponseBody: Body + DeserializeOwned {}

#[derive(Debug, Clone)]
pub enum HTTPMethod {
    Get,
    Post,
    Put,
}

#[derive(Component, Debug)]
pub struct Request<T, ResponseBody> {
    body: T,
    method: HTTPMethod,
    url: Url,

    _phantom: std::marker::PhantomData<ResponseBody>,
}

#[derive(Component, Debug)]
pub struct InProgress<Out> {
    pub(crate) receiver: Receiver<Result<Out, RequestError>>,
}

#[derive(Component, Debug)]
pub struct Response<T> {
    pub(crate) body: Result<T, RequestError>,
}

impl<T> Response<T> {
    pub fn body(&self) -> &Result<T, RequestError> {
        &self.body
    }
}

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
