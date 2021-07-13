//! Healthcheck-related routes.

use crate::context::Context;
use crate::pages::Page;
use maud::{html, Markup};
use rocket::async_stream::stream;
use rocket::futures::Stream;

/// Report the health of the application.
#[get("/healthcheck", format = "text/html")]
pub async fn render(context: Context) -> Page<impl Stream<Item = Markup>> {
    Page::builder(&context)
        .content(stream! {
            yield html! {
                h1 { "Healthy!" }
                p { "The application seems to be healthy." }
            };
        })
        .build()
}

#[cfg(test)]
mod tests {
    use crate::content_security_policy::ContentSecurityPolicy;

    use super::*;
    use rocket::{build, local::blocking::Client, uri};

    #[test]
    fn shows_healthy() {
        let client = Client::untracked(
            build()
                .attach(ContentSecurityPolicy::default())
                .mount("/", routes![render]),
        )
        .unwrap();
        let response = client.get(uri!(render)).dispatch();
        let body = response.into_string().unwrap();

        assert!(body.contains("Healthy!"));
    }
}
