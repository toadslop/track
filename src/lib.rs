//! The main application library. This is separated from the main binary to allow code
//! sharing between the main binary, the integration tests, and the benchmark suite.

pub mod app;
pub mod auth;
pub mod configuration;
pub mod database;
pub mod domain;
pub mod error;
mod middleware;
mod routes;
pub mod telemetry;

// Deps rexported so the utilities package can use them
pub use actix_web;
pub use anyhow;
pub use once_cell;
pub use uuid;
