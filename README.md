# tokio-binance
Unofficial async client for Binance.

[![Crates.io](https://img.shields.io/crates/v/tokio-binance.svg)](https://crates.io/crates/tokio-binance)
[![Documentation](https://docs.rs/tokio-binance/badge.svg)](https://docs.rs/tokio-binance)
![MIT/Apache-2 licensed](https://img.shields.io/crates/l/tokio-binance.svg)
[![Build Status](https://travis-ci.com/kgeronim/tokio-binance.svg?branch=master)](https://travis-ci.com/kgeronim/tokio-binance)

## Examples
Add this in your `Cargo.toml`:
```toml
[dependencies]
tokio-binance = "0.3"
serde_json = "1.0"
tokio = { version = "0.2", features = ["macros", "time"] }
```

#### Client
```rust
use tokio_binance::{AccountClient, BINANCE_US_URL, ID};
use serde_json::Value;
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AccountClient::connect("<api-key>", "<secret-key>", BINANCE_US_URL)?;
    let response = client
        .get_order("BNBUSDT", ID::ClientOId("<uuid>"))
        // optional: processing time for request; default is 5000, can't be above 60000.
        .with_recv_window(8000)
        //
        .json::<Value>()
        .await?;
    Ok(())
}
```

#### Websocket
```rust
use tokio_binance::*;
use tokio::time::{delay_for, Duration};
use serde_json::Value;
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = UserDataClient::connect("<api-key>", BINANCE_US_URL)?;
    let value = client.start_stream().json::<Value>().await?;
 
    let listen_key = value["listenKey"].as_str().unwrap();
    let listen_key_copy = listen_key.to_string();
 
    tokio::spawn(async move {
        loop {
            delay_for(Duration::from_secs(30*60)).await;
            if let Err(e) = client.keep_alive(&listen_key_copy).text().await {
                eprintln!("{}", e);
                return
            }
        }
    });
 
    let channel = Channel::UserData(listen_key);
    let mut stream = WebSocketStream::connect(channel, BINANCE_US_WSS_URL).await?;
 
    while let Some(value) = stream.json::<Value>().await? {
        if channel == value["stream"] {
            println!("{}", serde_json::to_string_pretty(&value)?);
        }
    }
    Ok(())
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.