use super::logout::*;
use rocket::{build, http::ContentType, local::blocking::Client, uri};

#[test]
fn shows_unimplemented_message() {
    let client = Client::untracked(build().mount("/", routes![render])).unwrap();
    let response = client
        .post(uri!(render))
        .header(ContentType::Form)
        .dispatch();
    let body = response.into_string().unwrap();

    assert!(body.contains("Not implemented."));
}
