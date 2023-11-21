//! The main application library. This is separated from the main binary to allow code
//! sharing between the main binary, the integration tests, and the benchmark suite.

pub mod app;
pub mod configuration;
pub mod database;
pub mod domain;
mod middleware;
mod routes;
pub mod telemetry;
