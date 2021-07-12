//! Routes used for self-service.

pub mod register;
#[cfg(test)]
mod register_test;

pub mod login;
#[cfg(test)]
mod login_test;

pub mod logout;
#[cfg(test)]
mod logout_test;
