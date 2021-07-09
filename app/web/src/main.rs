//! Web application for viewing RSS feeds.

#[macro_use]
extern crate rocket;

mod pages;

/// Initializes the async runtime and launches the web server.
#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![pages::healthcheck::render])
}
