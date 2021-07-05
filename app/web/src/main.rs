//! No-op web application.

#[macro_use]
extern crate rocket;

/// Initializes the async runtime and launches the web server.
#[launch]
async fn rocket() -> _ {
    rocket::build()
}
