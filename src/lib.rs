  
//! Unofficial async client for Binance.
//!
//! ## tokio_binance
//! 
//! ## Examples
//!
//! ### Async Client
//! ```no_run
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     Ok(())
//! }
//! ### Async Websocket
//! ```no_run
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     Ok(())
//! }
//! ```

mod param;
pub mod builder;
pub mod error;
mod client;
pub mod types;
mod ws_stream;

pub use self::ws_stream::*;
pub use self::param::*;
pub use self::client::*;