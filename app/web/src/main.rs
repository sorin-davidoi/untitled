//! Web application for viewing RSS feeds.

#[macro_use]
extern crate rocket;

mod pages;

/// Shared reqwest [`reqwest::Client`] instance intended to be used for all network operations.
static REQWEST_CLIENT: once_cell::sync::Lazy<reqwest::Client> = once_cell::sync::Lazy::new(|| {
    reqwest::Client::builder()
        .build()
        .expect("Could not construct reqwest client")
});

/// Initializes the async runtime and launches the web server.
#[launch]
async fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            pages::feed::view::render,
            pages::feed::view_item::render,
            pages::healthcheck::render
        ],
    )
}
