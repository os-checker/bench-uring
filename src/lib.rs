#[macro_use]
extern crate tracing;

pub mod client;
pub mod logger;
pub mod server;
pub mod utils;

pub type Result<T = (), E = eyre::Report> = std::result::Result<T, E>;
