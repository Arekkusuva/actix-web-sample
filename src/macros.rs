/// Adds resources in actix-web App.
///
/// ```rust
/// # use actix_web::{Responder, App, HttpRequest};
/// # use actix_web::http::Method;
/// # #[macro_use]
/// # mod macros;
/// #
/// fn get_users(_req: &HttpRequest) -> impl Responder {
///     format!("GET users")
//  }
///
/// # fn main() {
/// app = setup_routes!(App::new(), "users", [
///     ("/", Method::GET, get_users),
/// ]);
/// # }
/// ```
macro_rules! setup_routes {
    ($app:expr, $name:expr, [$(($path:expr, $method:expr, $handler:expr),)* $(,)*]$(,)*) => {{
        let mut r = std::collections::HashMap::new();
        $(
            r.entry($path.trim_end_matches("/"))
                .or_insert_with(|| Vec::with_capacity(1))
                .push(($method, $handler));
        )*
        for (path, handlers) in r {
            $app = $app.resource(&format!("/{}{}", $name, path), |r| {
                for (method, handler) in handlers {
                    r.method(method).f(handler);
                }
            });
        }
        $app
    }};
}
