use super::logout::*;
use crate::content_security_policy::ContentSecurityPolicy;
use rocket::{build, http::ContentType, local::blocking::Client, uri};

#[test]
fn shows_unimplemented_message() {
    let client = Client::untracked(
        build()
            .attach(ContentSecurityPolicy::default())
            .mount("/", routes![render]),
    )
    .unwrap();
    let response = client
        .post(uri!(render))
        .header(ContentType::Form)
        .dispatch();
    let body = response.into_string().unwrap();

    assert!(body.contains("Not implemented."));
}
