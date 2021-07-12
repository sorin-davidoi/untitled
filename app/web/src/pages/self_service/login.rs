//! Login-related routes.

use crate::pages::Page;
use maud::{html, Markup};
use rocket::async_stream::stream;
use rocket::form::Form;
use rocket::futures::Stream;

/// User credentials for authentication.
#[derive(FromForm)]
pub struct LoginCredentials<'r> {
    /// User's email;
    email: &'r str,
    #[allow(dead_code)]
    /// User's password
    password: &'r str,
}

/// Render the login form.
#[get("/self-service/login", format = "text/html")]
pub async fn render() -> Page<impl Stream<Item = Markup>> {
    Page::builder()
        .content(stream! {
            yield html! {
                main {
                    h1 { "Log in with your account" }
                    form action=(uri!(validate)) method="post" {
                        label {
                            "Email"
                            input name="email" r#type="email" required;
                        }
                        label {
                            "Password"
                            input name="password" r#type="password" required;
                        }
                        button { "Log in" };
                    }
                }
            };
        })
        .build()
}

/// Authenticate the user.
#[post(
    "/self-service/login",
    data = "<credentials>",
    format = "application/x-www-form-urlencoded"
)]
pub async fn validate(
    credentials: Form<LoginCredentials<'_>>,
) -> Page<impl Stream<Item = Markup> + '_> {
    Page::builder()
        .content(stream! {
            yield html! {
                main {
                    h1 { "Logging in with " (credentials.email) "..." }
                    p { "Not implemented." }
                }
            };
        })
        .build()
}
