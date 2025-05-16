use std::sync::atomic::AtomicUsize;

pub static COUNT: AtomicUsize = AtomicUsize::new(0);

pub const ADDR: &str = "127.0.0.1:2345";
/// How many bytes to be transmitted.
pub const SIZE: usize = 16 * 1024;
/// How long the server last.
pub const DURATION: Duration = Duration::from_secs(10);

pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub use std::net::SocketAddr;
pub use std::sync::atomic::Ordering;
pub use std::time::{Duration, Instant};
