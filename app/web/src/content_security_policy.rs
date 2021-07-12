//! Content Security Policy related types and structures.

use rocket::fairing::{Fairing, Info, Kind};
use rocket::{http::Header, Data, Request, Response};

/// Fairing for adding the [`Content-Security-Policy` header](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Security-Policy) to responses.
///
/// For each request it:
/// - generates a cryptographic nonce using [`rand`]
/// - places the nonce in the request's local cache
/// - when the response is generated it fetches the nonce from the cache and uses it to construct the `Content-Security-Policy` header, which is added to the response
///
/// Note that there may other request guards that fetch the nonce from the request's local cache (e.g. [`Context`](`crate::context::Context`)).
pub struct ContentSecurityPolicy;

/// [Cryptographic nonce](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/nonce) (number used once) suitable to be used by [Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP).
#[derive(Clone, Debug)]
pub struct Nonce(String);

impl std::fmt::Display for Nonce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for ContentSecurityPolicy {
    fn default() -> Self {
        Self {}
    }
}

#[rocket::async_trait]
impl Fairing for ContentSecurityPolicy {
    fn info(&self) -> Info {
        Info {
            name: "Content Security Policy",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        req.local_cache(|| Nonce(rand::random::<u64>().to_string()));
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        // The closure won't be executed since self::on_request is guaranteed to have executed by this point.
        let nonce = req.local_cache::<Nonce, _>(|| unreachable!());

        res.set_header(Header::new(
            "Content-Security-Policy",
            format!("base-uri 'none'; default-src 'none'; frame-ancestors 'none'; manifest-src 'none'; img-src 'self'; script-src 'none'; worker-src 'none'; connect-src 'none'; style-src 'nonce-{}'; form-action 'self';", nonce)
        ));
    }
}
