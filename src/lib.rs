pub mod server;

pub type Result = std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;
