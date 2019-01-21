use actix_web::{Result, HttpResponse};
use actix_web::middleware::{Started, Finished, Middleware};
use time::precise_time_ns;

use crate::SharedState;
use crate::api::prelude::LoggerReqExt;
use crate::api::Request;

pub struct ResponseLoggerMiddleware;

struct StartTime(u64);

impl Middleware<SharedState> for ResponseLoggerMiddleware {
    fn start(&self, req: &Request) -> Result<Started> {
        req.extensions_mut().insert(StartTime(precise_time_ns()));
        Ok(Started::Done)
    }

    fn finish(&self, req: &Request, resp: &HttpResponse) -> Finished {
        let start_time: StartTime = req.extensions_mut().remove().unwrap();
        let delta = (precise_time_ns() - start_time.0) as f64 / 1_000_000.0;

        info!(
            req.logger(),
            "Request with response status: {}, took: {} ms",
            resp.status(), delta,
        );
        Finished::Done
    }
}
