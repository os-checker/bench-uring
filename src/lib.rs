#[macro_use]
extern crate tracing;

pub mod client;
pub mod logger;
pub mod server;

pub type Result<T = (), E = eyre::Report> = std::result::Result<T, E>;

pub mod utils {
    pub use crate::Result;
    pub use eyre::{Context, ContextCompat};
    pub use std::time::Duration;
    pub use tracing::Instrument;

    /// All examples are run sequentially, so this addr is used in single server in each run.
    pub const ADDR: &str = "127.0.0.1:2345";
    /// How many bytes to be transmitted.
    pub const SIZE: usize = 16 * 1024;
    /// Real data to be transmitted.
    pub const DATA: &[u8] = &[0; SIZE];
    /// How many socket to connect to the server.
    pub const LEN: usize = 100;
    /// How long the server last.
    pub const DURATION: Duration = Duration::from_secs(4);
}
