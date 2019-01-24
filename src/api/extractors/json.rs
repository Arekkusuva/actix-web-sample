use actix_web::{HttpRequest, Json as ActixJson, Error, FromRequest};
use actix_web::dev::JsonConfig;
use serde::de::DeserializeOwned;
use futures::Future;
use validator::Validate;

use std::ops::{Deref, DerefMut};
use std::fmt;

use crate::api::transport::Response;

pub struct Json<T>(T);

impl<T> Json<T> {
    #[allow(dead_code)]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> fmt::Debug for Json<T>
    where
        T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Json: {:?}", self.0)
    }
}

impl<T> fmt::Display for Json<T>
    where
        T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<T, S> FromRequest<S> for Json<T>
    where
        T: DeserializeOwned + Validate +'static,
        S: 'static,
{
    type Config = JsonConfig<S>;
    type Result = Box<Future<Item = Self, Error = Error>>;

    #[inline]
    fn from_request(req: &HttpRequest<S>, cfg: &Self::Config) -> Self::Result {
        Box::new(ActixJson::from_request(req, cfg).and_then(|json: ActixJson<T>| {
            match json.validate() {
                Ok(_) => Ok(Json(json.into_inner())),
                Err(e) => Err(Error::from(Response::from(e))),
            }
        }))
    }
}