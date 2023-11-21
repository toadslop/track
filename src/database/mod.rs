//! A module for abstracting over the database connection.

mod client;
mod error;
mod init;

pub use client::Database;
pub use init::init;
