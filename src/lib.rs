pub mod client;
pub mod server;

pub type Result<T = (), E = Box<dyn std::error::Error + Send + Sync>> = std::result::Result<T, E>;

pub mod utils {
    pub use crate::Result;
    pub const ADDR: &str = "127.0.0.1:2345";
    /// how many bytes to be transmitted.
    pub const SIZE: usize = 16 * 1024;
    /// real data to be transmitted.
    pub const DATA: &[u8] = &[0; SIZE];
    /// how many socket to connect to the server.
    pub const LEN: usize = 100;
}
