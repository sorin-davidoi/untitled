//! Web application for viewing RSS feeds.

#[macro_use]
extern crate rocket;

mod content_security_policy;
mod context;
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
    rocket::build()
        .attach(content_security_policy::ContentSecurityPolicy::default())
        .mount(
            "/",
            routes![
                pages::self_service::register::render,
                pages::self_service::register::create,
                pages::self_service::login::render,
                pages::self_service::login::validate,
                pages::self_service::logout::render,
                pages::feed::view::render,
                pages::feed::view_item::render,
                pages::healthcheck::render
            ],
        )
}
