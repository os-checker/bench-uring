pub use crate::Result;
pub use eyre::{Context, ContextCompat};
pub use std::time::Duration;
pub use tracing::Instrument;

// ******** Default Configs ********

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

// ******** Configs Environment Variables ********

pub const ENV_ADDR: &str = "ADDR";
pub const ENV_SIZE: &str = "SIZE";
pub const ENV_SOCKET_LEN: &str = "SOCKET_LEN";
pub const ENV_DURATION: &str = "DURATION";
pub const ENV_INTERVAL: &str = "INTERVAL";

// ******** Configs Static Used in Server and Client ********

use std::sync::LazyLock;
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    use std::env::var;
    Config {
        addr: var(ENV_ADDR).unwrap_or_else(|_| ADDR.to_owned()),
        size: var(ENV_SIZE)
            .map(|size| size.parse().unwrap())
            .unwrap_or(SIZE),
        data: DATA,
        socket_len: var(ENV_SOCKET_LEN)
            .map(|size| size.parse().unwrap())
            .unwrap_or(SOCKET_LEN),
        duration: var(ENV_DURATION)
            .map(|dur| Duration::from_secs(dur.parse::<u64>().unwrap()))
            .unwrap_or(DURATION),
        interval: var(ENV_INTERVAL)
            .map(|dur| Duration::from_secs(dur.parse::<u64>().unwrap()))
            .unwrap_or(INTERVAL),
    }
});

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
