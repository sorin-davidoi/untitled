use super::login::*;
use crate::content_security_policy::ContentSecurityPolicy;
use rocket::{build, http::ContentType, local::blocking::Client, uri};

#[test]
fn shows_form() {
    let client = Client::untracked(
        build()
            .attach(ContentSecurityPolicy::default())
            .mount("/", routes![render]),
    )
    .unwrap();
    let response = client.get(uri!(render)).dispatch();
    let body = response.into_string().unwrap();

    assert!(body.contains("<form"));
}

#[test]
fn shows_unimplemented_message() {
    let client = Client::untracked(
        build()
            .attach(ContentSecurityPolicy::default())
            .mount("/", routes![validate]),
    )
    .unwrap();
    let response = client
        .post(uri!(validate))
        .header(ContentType::Form)
        .body("email=test_email@test_email.com&password=test_password")
        .dispatch();
    let body = response.into_string().unwrap();

    assert!(body.contains("Not implemented."));
}
