
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::error::Error as ActixWebError;
use actix_web::http::StatusCode;
use serde::{Serialize, Serializer};

use crate::SharedState;

const APPLICATION_JSON: &'static str = "application/json";

pub type Request = HttpRequest<SharedState>;

pub struct ResponseStatus(pub StatusCode);

impl Serialize for ResponseStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let code = self.0.as_u16();
        if code >= 200 && code < 300 {
            return serializer.serialize_str("ok")
        }
        serializer.serialize_str("error")
    }
}

#[derive(Serialize)]
pub struct Response {
    status: ResponseStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl Response {
    pub fn new(status: ResponseStatus) -> Self {
        Self {
            status,
            error: None,
        }
    }

    pub fn with_error(status: ResponseStatus, msg: &str) -> Self {
        Self {
            status,
            error: Some(msg.to_owned()),
        }
    }
}

impl Responder for Response {
    type Item = HttpResponse;
    type Error = ActixWebError;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<Self::Item, Self::Error> {
        let body = serde_json::to_string(&self)?;

        Ok(req
            .build_response(self.status.0)
            .content_type(APPLICATION_JSON)
            .body(body))
    }
}