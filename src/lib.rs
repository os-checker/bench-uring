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
    const ADDR: &str = "127.0.0.1:2345";
    /// How many bytes to be transmitted.
    const SIZE: usize = 16 * 1024;
    /// Real data to be transmitted.
    const DATA: &[u8] = &[0; SIZE];
    /// How many socket to connect to the server.
    const SOCKET_LEN: usize = 100;
    /// How long the server lasts.
    const DURATION: Duration = Duration::from_secs(4);
    /// How often a tick occurs to metric.
    const INTERVAL: Duration = Duration::from_secs(2);

    pub struct Config {
        /// All examples are run sequentially, so this addr is used in single server in each run.
        pub addr: String,
        /// How many bytes to be transmitted.
        pub size: usize,
        /// Real data to be transmitted.
        pub data: &'static [u8],
        /// How many socket to connect to the server.
        pub socket_len: usize,
        /// How long the server lasts.
        pub duration: Duration,
        /// How often a tick occurs to metric.
        pub interval: Duration,
    }

    use std::sync::LazyLock;
    pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
        use std::env::var;
        Config {
            addr: var("ADDR").unwrap_or_else(|_| ADDR.to_owned()),
            size: var("SIZE")
                .map(|size| size.parse().unwrap())
                .unwrap_or(SIZE),
            data: DATA,
            socket_len: var("SOCKET_LEN")
                .map(|size| size.parse().unwrap())
                .unwrap_or(SOCKET_LEN),
            duration: var("DURATION")
                .map(|dur| Duration::from_secs(dur.parse::<u64>().unwrap()))
                .unwrap_or(DURATION),
            interval: var("INTERVAL")
                .map(|dur| Duration::from_secs(dur.parse::<u64>().unwrap()))
                .unwrap_or(INTERVAL),
        }
    });
}
