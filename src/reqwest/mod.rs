use crossbeam_channel::Receiver; // Cannot use mpsc because mpsc::Receiver is not Sync

use bevy::{
    ecs::system::SystemParam,
    prelude::{Commands, Component, Entity, Res},
};
use reqwest::{RequestBuilder, Url};
use serde::{de::DeserializeOwned, Serialize};

use self::plugin::ReqwestClient;

mod plugin;

pub trait Body: 'static + Send + Sync + Clone {}

pub trait RequestBody: Body + Serialize {}
pub trait ResponseBody: Body + DeserializeOwned {}

#[derive(SystemParam)]
pub struct ReqwestRunner<'w, 's> {
    commands: Commands<'w, 's>,
    client: Res<'w, ReqwestClient>,
}

impl<'w, 's> ReqwestRunner<'w, 's> {
    pub fn client(&self) -> &reqwest::Client {
        &self.client.client
    }

    pub fn request<T>(&mut self, request: RequestBuilder) -> ReqwestHandle<T>
    where
        T: ResponseBody,
    {
        ReqwestHandle {
            entity: self.commands.spawn(Request::<T>::new(request)).id(),
            _phantom: Default::default(),
        }
    }
}

pub struct ReqwestHandle<T> {
    entity: Entity,
    _phantom: std::marker::PhantomData<T>,
}

#[derive(Debug, Clone)]
pub enum HTTPMethod {
    Get,
    Post,
    Put,
}

#[derive(Component, Debug)]
pub(crate) struct Request<ResponseBody> {
    builder: Option<RequestBuilder>,
    _phantom: std::marker::PhantomData<ResponseBody>,
}

impl<ResponseBody> Request<ResponseBody> {
    pub(crate) fn new(builder: RequestBuilder) -> Self {
        Self {
            builder: Some(builder),
            _phantom: std::marker::PhantomData,
        }
    }
}

// #[derive(Component, Debug)]
// pub(crate) struct Request<T, ResponseBody> {
//     body: T,
//     method: HTTPMethod,
//     url: Url,

//     _phantom: std::marker::PhantomData<ResponseBody>,
// }

#[derive(Component, Debug)]
pub(crate) struct InProgress<T> {
    pub receiver: Receiver<Result<T, RequestError>>,
}

#[derive(Component, Debug)]
pub(crate) struct Response<T> {
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
