use super::view::*;
use crate::content_security_policy::ContentSecurityPolicy;
use mockito::{mock, server_url};
use rocket::{build, http::Status, local::blocking::Client, uri};

#[test]
fn shows_error_message_for_malformed_uri() {
    let client = Client::untracked(
        build()
            .attach(ContentSecurityPolicy::default())
            .mount("/", routes![render]),
    )
    .unwrap();
    let response = client.get(uri!(render(uri = "test"))).dispatch();
    let body = response.into_string().unwrap();

    assert!(body.contains("Could not initiate connection:"));
}

#[test]
fn shows_error_message_for_request_error() {
    let client = Client::untracked(
        build()
            .attach(ContentSecurityPolicy::default())
            .mount("/", routes![render]),
    )
    .unwrap();
    let mock = mock("GET", "/")
        .expect(10)
        .with_status(Status::TemporaryRedirect.code.into())
        .with_header("Location", &server_url())
        .create();
    let response = client.get(uri!(render(uri = server_url()))).dispatch();
    let body = response.into_string().unwrap();

    mock.assert();
    assert!(dbg!(body).contains("Could not establish connection:"));
}

#[test]
fn shows_error_message_for_malformed_feed() {
    let client = Client::untracked(
        build()
            .attach(ContentSecurityPolicy::default())
            .mount("/", routes![render]),
    )
    .unwrap();
    let mock = mock("GET", "/").expect(1).create();
    let response = client.get(uri!(render(uri = server_url()))).dispatch();
    let body = response.into_string().unwrap();

    mock.assert();
    assert!(body.contains("Could not parse feed:"));
}

#[test]
fn shows_feed() {
    let client = Client::untracked(
        build()
            .attach(ContentSecurityPolicy::default())
            .mount("/", routes![render]),
    )
    .unwrap();
    let mock = mock("GET", "/")
        .with_body(
            r#"
        <?xml version="1.0" encoding="UTF-8"?>
            <rss version="2.0">
                <channel>
                    <title>Mocked title</title>
                </channel>
                <item>
                    <title>Mocked first item</title>
                </item>
            </rss>
        </xml>
    "#,
        )
        .expect(1)
        .create();
    let response = client.get(uri!(render(uri = server_url()))).dispatch();
    let body = response.into_string().unwrap();

    mock.assert();
    assert!(body.contains("Mocked title"));
    assert!(body.contains("Mocked first item"));
}
