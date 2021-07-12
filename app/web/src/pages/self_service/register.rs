//! Registration-related routes.

use crate::pages::Page;
use maud::{html, Markup};
use rocket::async_stream::stream;
use rocket::form::Form;
use rocket::futures::Stream;

/// User credentials for creating a new account.
#[derive(FromForm)]
pub struct CreateAccountCredentials<'r> {
    /// User's email.
    email: &'r str,
    /// User's password.
    #[allow(dead_code)]
    password: &'r str,
}

/// Report the health of the application.
#[get("/self-service/create-account", format = "text/html")]
pub async fn render() -> Page<impl Stream<Item = Markup>> {
    Page::builder()
        .content(stream! {
            yield html! {
                main {
                    h1 { "Create account" }
                    form action=(uri!(create)) method="post" {
                        label {
                            "Email"
                            input name="email" type="email" required;
                        }
                        label {
                            "Password"
                            input name="password" r#type="password" required;
                        }
                        button { "Create account" };
                    }
                }
            };
        })
        .build()
}

/// Create account with the provided credentials.
#[post(
    "/self-service/create-account",
    data = "<credentials>",
    format = "application/x-www-form-urlencoded"
)]
pub async fn create(
    credentials: Form<CreateAccountCredentials<'_>>,
) -> Page<impl Stream<Item = Markup> + '_> {
    Page::builder()
        .content(stream! {
            yield html! {
                main {
                    h1 { "Creating account with " (credentials.email) "..." }
                    p { "Not implemented." }
                }
            };
        })
        .build()
}
