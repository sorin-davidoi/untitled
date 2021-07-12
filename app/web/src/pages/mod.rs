//! Routes that render HTML and utility function for easy integration with Rocket.

pub mod feed;
pub mod healthcheck;
pub mod self_service;

mod page;

pub use page::Page;
