#[cfg(feature = "runtime")]
pub use wasmtime_wasi_http::body::HyperIncomingBody as Body;

pub mod app_info;
pub mod config;
pub mod routes;
pub mod trigger;
#[cfg(feature = "runtime")]
pub mod wagi;

pub const WELL_KNOWN_PREFIX: &str = "/.well-known/spin/";

#[cfg(feature = "runtime")]
pub mod body {
    use super::Body;
    use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
    use hyper::body::Bytes;

    pub fn full(bytes: Bytes) -> Body {
        BoxBody::new(Full::new(bytes).map_err(|_| unreachable!()))
    }

    pub fn empty() -> Body {
        BoxBody::new(Empty::new().map_err(|_| unreachable!()))
    }
}
