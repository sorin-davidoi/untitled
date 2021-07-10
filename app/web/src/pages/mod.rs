//! Routes that render HTML and utility function for easy integration with Rocket.

pub mod feed;
pub mod healthcheck;

mod page;

pub use page::Page;
