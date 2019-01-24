
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web::error::Error as ActixWebError;
use actix_web::http::StatusCode;
use serde::{Serialize, Serializer};
use serde_json::Value as JsonValue;
use validator::ValidationErrors;

use std::convert::Into;
use std::error::Error;
use std::fmt;

pub mod users;

use crate::SharedState;
use crate::db::DbError;

pub type Request = HttpRequest<SharedState>;

pub type ResponseResult = std::result::Result<Response, Response>;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<JsonValue>,
}

impl Response {
    pub fn new(status: StatusCode) -> Self {
        Self {
            status: ResponseStatus(status),
            error: None,
            data: None,
        }
    }

    pub fn with_error(status: StatusCode, msg: &str) -> Self {
        Self {
            status: ResponseStatus(status),
            error: Some(msg.to_owned()),
            data: None,
        }
    }

    pub fn data<T: Into<JsonValue>>(mut self, data: T) -> Self {
        self.data = Some(data.into());
        self
    }

    pub fn status_code(&self) -> StatusCode {
        self.status.0
    }

    pub fn as_str(&self) -> &'static str {
        self.status_code().canonical_reason().unwrap()
    }

    pub fn as_http_resp(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(self)
    }
}

impl From<Response> for HttpResponse {
    fn from(resp: Response) -> Self {
        resp.as_http_resp()
    }
}

impl Responder for Response {
    type Item = HttpResponse;
    type Error = ActixWebError;

    fn respond_to<S>(self, _: &HttpRequest<S>) -> Result<Self::Item, Self::Error> {
        Ok(self.into())
    }
}

impl From<DbError> for Response {
    fn from(err: DbError) -> Self {
        match err {
            DbError::AlreadyExist => Response::with_error(
                StatusCode::CONFLICT,
                StatusCode::CONFLICT.canonical_reason().unwrap()),
            _ => Response::with_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                StatusCode::INTERNAL_SERVER_ERROR.canonical_reason().unwrap()),
        }
    }
}

// TODO: Finish it
impl From<ValidationErrors> for Response {
    fn from(_err: ValidationErrors) -> Self {
        Response::new(StatusCode::BAD_REQUEST)
    }
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Error for Response {}

impl ResponseError for Response {
    fn error_response(&self) -> HttpResponse {
        self.as_http_resp()
    }
}
