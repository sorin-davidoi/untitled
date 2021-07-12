//! Utility structures for rendering HTML with Rocket.

use maud::{html, Markup, PreEscaped, DOCTYPE};

use rocket::async_stream::stream;
use rocket::futures::{Stream, StreamExt};
use rocket::http::{ContentType, Status};
use rocket::response::stream::ReaderStream;
use rocket::response::Responder;
use rocket::{Request, Response};

/// Main unit of rendering HTML. Usually constructed with [`Self::builder()`].
pub struct Page<S> {
    /// The main content of the page.
    pub content: S,
}

impl<'r, S: Stream<Item = Markup> + Send> Page<S> {
    /// Constructs a new builder instance.
    pub fn builder() -> PageBuilder<S> {
        PageBuilder::default()
    }
}

impl<'r, S: Stream<Item = Markup>> Responder<'r, 'r> for Page<S>
where
    S: Send + 'r,
    S::Item: Send + Unpin + 'r,
{
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'r> {
        Response::build()
            .status(Status::Ok)
            .header(ContentType::HTML)
            .streamed_body(ReaderStream::from(
                self.content
                    .map(|pre_escaped| pre_escaped.into_string())
                    .map(std::io::Cursor::new),
            ))
            .ok()
    }
}

/// Builder for [`Page`]. Takes care of constructing a page with valid HTML by wrapping the content with the appropriate markup.
pub struct PageBuilder<S: Stream<Item = Markup>>
where
    S: Send,
    S::Item: Send + Unpin,
{
    /// The main content of the page.
    content: Option<S>,
}

impl<S: Stream<Item = Markup>> Default for PageBuilder<S>
where
    S: Send,
    S::Item: Send + Unpin,
{
    fn default() -> Self {
        Self { content: None }
    }
}

impl<'r, S: Stream<Item = Markup>> PageBuilder<S>
where
    S: Send + 'r,
    S::Item: Send + Unpin + 'r,
{
    /// Sets the main content of the page.
    pub fn content(mut self, content: S) -> Self {
        self.content = Some(content);
        self
    }

    /// Finalize the builder.
    pub fn build(self) -> Page<impl Stream<Item = Markup> + 'r> {
        let pre = self.pre();
        let post = self.post();
        Page {
            content: match self.content {
                Some(content) => stream! { yield pre; }
                    .chain(content)
                    .chain(stream! { yield post; }),
                None => unimplemented!(),
            },
        }
    }

    /// Render the beginning of the main document (the part _before_ the actual page content). The rest is rendered in [`Self::post()`].
    fn pre(&self) -> Markup {
        html! {
            (DOCTYPE)
            html lang="en";
                head {
                    meta charset="UTF-8";
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    title { "Untitled" }
                }
                body;
                    nav {
                        a href=(uri!(super::self_service::register::render)) { "Create account"}
                        a href=(uri!(super::self_service::login::render)) { "Log in" }
                        form action=(uri!(super::self_service::logout::render)) method="post" {
                            button { "Logout" }
                        }
                    }

        }
    }

    /// Render the end of the main document (the part _after_ the actual page content). The markup must correctly close the tags opened in [`Self::pre()`].
    fn post(&self) -> Markup {
        html! {
                (PreEscaped("</body>"));
            (PreEscaped("</html>"));
        }
    }
}
