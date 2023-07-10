use bevy::{
    ecs::{schedule::ScheduleLabel, system::SystemParam},
    prelude::{Commands, Entity, Query, Res, SystemSet},
};
use bytes::Bytes;
use plugin::{HttpRequest, HttpRequestClient, HttpResponseReceived};
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};

mod plugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, ScheduleLabel)]
pub struct ExecuteHttpRequests;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum HttpRequestSet {
    BeforeExecuteRequests,
    ExecuteRequests,
    AfterExecuteRequests,
    GatherResponses,
    AfterGatherResponses,
}

pub struct HttpRequestPlugin;

#[derive(SystemParam)]
pub struct HttpRequests<'w, 's> {
    commands: Commands<'w, 's>,
    client: Res<'w, HttpRequestClient>,
    response_received_query: Query<'w, 's, &'static HttpResponseReceived>,
}

impl<'w, 's> HttpRequests<'w, 's> {
    #[must_use]
    pub fn client(&self) -> &Client {
        &self.client.client
    }

    #[must_use]
    pub fn request(&mut self, request: RequestBuilder) -> HttpRequestHandle {
        HttpRequestHandle {
            entity: self.commands.spawn(HttpRequest::new(request)).id(),
        }
    }

    #[must_use]
    pub fn get_response(
        &self,
        handle: &HttpRequestHandle,
    ) -> Option<&Result<HttpResponse, HttpRequestError>> {
        if let Ok(response) = self.response_received_query.get(handle.entity) {
            Some(&response.result)
        } else {
            None
        }
    }

    pub fn dispose(&mut self, handle: HttpRequestHandle) {
        self.commands.entity(handle.entity).despawn();
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct HttpRequestHandle {
    entity: Entity,
}

#[derive(Debug)]
pub struct HttpResponse {
    body_bytes: Bytes,
    status_code: StatusCode,
}

impl HttpResponse {
    #[must_use]
    pub fn body_bytes(&self) -> &Bytes {
        &self.body_bytes
    }

    #[must_use]
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    #[must_use]
    pub fn json<'de, T>(&'de self) -> Result<T, serde_json::Error>
    where
        T: Deserialize<'de>,
    {
        serde_json::from_slice(&self.body_bytes)
    }
}

#[derive(Debug, Clone)]
pub enum HttpRequestError {
    RequestFailed(String),
}

impl From<reqwest::Error> for HttpRequestError {
    fn from(error: reqwest::Error) -> Self {
        Self::RequestFailed(format!("{error:?}"))
    }
}
