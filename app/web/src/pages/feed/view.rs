//! Route for viewing feed information.

use maud::{html, Markup, PreEscaped};
use rocket::async_stream::stream;
use rocket::futures::Stream;
use rocket::tokio::task::spawn_blocking;
use rocket::uri;
use rss::Channel;
use std::io::Cursor;

use crate::context::Context;
use crate::pages::Page;
use crate::REQWEST_CLIENT;

/// Render the feed at the given URI.
#[get("/feeds/<uri>", format = "text/html")]
pub async fn render<'r>(uri: String, context: Context) -> Page<impl Stream<Item = Markup>> {
    Page::builder(&context)
        .content(stream! {
            yield html! { style nonce=(context.nonce); };
            yield html! { (PreEscaped("
                main > [role='progressbar'] { display: inline-flex; justify-content: center; }
                main > [role='progressbar']:not(:last-child),
                main > [role='progressbar'] > span:not(:last-child) { display: none; }
            ")) };
            yield html! { (PreEscaped("</style>")) };
            yield html! { main; };
            yield html! { div role="progressbar" aria-label=(&uri); };
            yield html! { span { "Establishing connection..." } };

            let client = REQWEST_CLIENT.clone();

            let request = match client.get(&uri).build() {
                Ok(request) => request,
                Err(err) => {
                    yield html! { span { "Could not initiate connection: " (err) } };
                    yield html! { (PreEscaped("</div></main>")) };
                    return;
                },
            };

            let response = match client.execute(request).await {
                Ok(response) => response,
                Err(err) => {
                    yield html! { span { "Could not establish connection: " (err) } };
                    yield html! { (PreEscaped("</div></main>")) };
                    return;
                }
            };

            match response.status().is_success() {
                true => {
                    yield html! { span { "Fetching content..." } };
                }
                false => {
                    yield html! { span { "Server returned error: " (response.status()) } };
                    yield html! { (PreEscaped("</div></main>")) };
                    return;
                }
            };

            let bytes = match response.bytes().await {
                Ok(bytes) => {
                    yield html! { span { "Parsing content..." } };
                    bytes
                }
                Err(err) => {
                    yield html! { span { "Could not fetch content: " (err) } };
                    yield html! { (PreEscaped("</div></main>")) };
                    return;
                }
            };

            match spawn_blocking(move || Channel::read_from(Cursor::new(bytes))).await {
                Ok(Ok(feed)) => {
                    yield html! {
                        (PreEscaped("</div>"))
                        h1 { (feed.title()) }
                        p { (feed.description()) }
                    };

                    yield html! { ol; };
                    for item in feed.items() {
                        yield html! {
                            li {
                                @if let Some(guid) = item.guid() {
                                    a href=(uri!(super::view_item::render(uri = &uri, guid = guid.value()))) { (item.title().unwrap_or_else(|| "Untitled")) };
                                } @else {
                                    (item.title().unwrap_or_else(|| "Untitled"))
                                }
                            }
                        }
                    }
                    yield html! { (PreEscaped("</ol>")) };
                },
                Ok(Err(err)) => {
                    yield html! { span { "Could not parse feed: " (err) } };
                    yield html! { (PreEscaped("</div></main>")) };
                    return;
                },
                Err(err) => {
                    yield html! { span { "Something went wrong while parsing the feed: " (err) } };
                    yield html! { (PreEscaped("</div></main>")) };
                    return;
                },
            }

            yield html! { (PreEscaped("</main>")) };
        })
        .build()
}
