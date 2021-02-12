//! Unofficial async client for Binance.
//!
//! Simple and easy to use wrapper for the Binance API.
//! ## Examples
//!
//! ### Client
//! ```no_run
//! use tokio_binance::{AccountClient, BINANCE_US_URL, ID};
//! use serde_json::Value;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = AccountClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
//!     let response = client
//!         .get_order("BNBUSDT", ID::ClientOId("<uuid>"))
//!         // optional: processing time for request; default is 5000, can't be above 60000.
//!         .with_recv_window(8000)
//!         //
//!         .json::<Value>()
//!         .await?;
//!     Ok(())
//! }
//! ```
//! ### Websocket
//! ```no_run
//! use tokio_binance::*;
//! use tokio::time::Duration;
//! use serde_json::Value;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = UserDataClient::connect("<api-key>", BINANCE_US_URL)?;
//!     let value = client.start_stream().json::<Value>().await?;
//!
//!     let listen_key = value["listenKey"].as_str().unwrap();
//!     let listen_key_copy = listen_key.to_string();
//!
//!     tokio::spawn(async move {
//!         loop {
//!             std::thread::sleep(Duration::from_secs(30*60));
//!             if let Err(e) = client.keep_alive(&listen_key_copy).text().await {
//!                 eprintln!("{}", e);
//!                 return
//!             }
//!         }
//!     });
//!
//!     let channel = Channel::UserData(listen_key);
//!     let mut stream = WebSocketStream::connect(channel, BINANCE_US_WSS_URL).await?;
//!
//!     while let Some(value) = stream.json::<Value>().await? {
//!         if channel == value["stream"] {
//!             println!("{}", serde_json::to_string_pretty(&value)?);
//!         }
//!     }
//!     Ok(())
//! }
//! ```

pub mod builder;
mod client;
pub mod error;
mod param;
pub mod types;
mod ws_stream;

pub use self::client::*;
pub use self::param::*;
pub use self::ws_stream::*;
