use bevy::{
    ecs::system::SystemParam,
    prelude::{Commands, Entity, Query, Res},
};
use bytes::Bytes;
use plugin::{HttpRequest, HttpRequestClient, HttpResponseReceived};
use reqwest::{Client, RequestBuilder, StatusCode};

mod plugin;

pub struct HttpRequestPlugin;

#[derive(SystemParam)]
pub struct HttpRequests<'w, 's> {
    commands: Commands<'w, 's>,
    client: Res<'w, HttpRequestClient>,
    response_received_query: Query<'w, 's, &'static HttpResponseReceived>,
}

impl<'w, 's> HttpRequests<'w, 's> {
    pub fn client(&self) -> &Client {
        &self.client.client
    }

    pub fn request(&mut self, request: RequestBuilder) -> HttpRequestHandle {
        HttpRequestHandle {
            entity: self.commands.spawn(HttpRequest::new(request)).id(),
        }
    }

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

#[derive(Debug)]
pub struct HttpRequestHandle {
    entity: Entity,
}

#[derive(Debug)]
pub struct HttpResponse {
    body_bytes: Bytes,
    status_code: StatusCode,
}

impl HttpResponse {
    pub fn body_bytes(&self) -> &Bytes {
        &self.body_bytes
    }

    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }
}

#[derive(Debug)]
pub enum HttpRequestError {
    RequestFailed(reqwest::Error),
}

impl From<reqwest::Error> for HttpRequestError {
    fn from(error: reqwest::Error) -> Self {
        Self::RequestFailed(error)
    }
}
