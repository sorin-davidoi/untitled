//! Logout-related routes.

use crate::pages::Page;
use maud::{html, Markup};
use rocket::async_stream::stream;
use rocket::futures::Stream;

/// Logs out the user.
#[post("/self-service/logout", format = "application/x-www-form-urlencoded")]
pub async fn render() -> Page<impl Stream<Item = Markup>> {
    Page::builder()
        .content(stream! {
            yield html! {
                main {
                    h1 { "Log out" }
                    p { "Not implemented." }
                }
            };
        })
        .build()
}
