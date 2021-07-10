use super::view_item::*;
use mockito::{mock, server_url};
use rocket::{build, http::Status, local::blocking::Client, uri};

#[test]
fn shows_error_message_for_malformed_uri() {
    let client = Client::untracked(build().mount("/", routes![render])).unwrap();
    let response = client
        .get(uri!(render(uri = "test", guid = "test")))
        .dispatch();
    let body = response.into_string().unwrap();

    assert!(body.contains("Could not initiate connection:"));
}

#[test]
fn shows_error_message_for_request_error() {
    let client = Client::untracked(build().mount("/", routes![render])).unwrap();
    let mock = mock("GET", "/")
        .expect(10)
        .with_status(Status::TemporaryRedirect.code.into())
        .with_header("Location", &server_url())
        .create();
    let response = client
        .get(uri!(render(uri = server_url(), guid = "test")))
        .dispatch();
    let body = response.into_string().unwrap();

    mock.assert();
    assert!(dbg!(body).contains("Could not establish connection:"));
}

#[test]
fn shows_error_message_for_malformed_feed() {
    let client = Client::untracked(build().mount("/", routes![render])).unwrap();
    let mock = mock("GET", "/").expect(1).create();
    let response = client
        .get(uri!(render(uri = server_url(), guid = "test")))
        .dispatch();
    let body = response.into_string().unwrap();

    mock.assert();
    assert!(body.contains("Could not parse feed:"));
}

#[test]
fn shows_feed_item() {
    let client = Client::untracked(build().mount("/", routes![render])).unwrap();
    let mock = mock("GET", "/")
        .with_body(
            r#"
        <?xml version="1.0" encoding="UTF-8"?>
            <rss version="2.0">
                <channel>
                    <title>Mocked title</title>
                </channel>
                <item>
                    <guid>test</guid>
                    <title>Mocked first item title</title>
                    <description>Mocked first item description</description>
                    <content:encoded>Mocked first item content</content:encoded>
                </item>
            </rss>
        </xml>
    "#,
        )
        .expect(1)
        .create();
    let response = client
        .get(uri!(render(uri = server_url(), guid = "test")))
        .dispatch();
    let body = response.into_string().unwrap();

    mock.assert();
    assert!(body.contains("Mocked first item title"));
    assert!(body.contains("Mocked first item description"));
    assert!(body.contains("Mocked first item content"));
}
