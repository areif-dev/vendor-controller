pub mod controller;
pub mod product;

use std::time::Duration;

pub use controller::VendorController;
pub use product::Product;
pub use product::parse_price_nonstrict;
use serde_json::Map;

#[derive(Debug)]
pub struct ChromeClient {
    pub client: fantoccini::Client,
    pub wait_at_most: Duration,
}

impl ChromeClient {
    pub async fn init(
        port: u16,
        wait_at_most: Duration,
    ) -> Result<Self, fantoccini::error::NewSessionError> {
        let mut caps = Map::new();
        #[cfg(debug_assertions)]
        let options = serde_json::json!({
            "args": ["--window-size=1920,1080"]
        });
        #[cfg(not(debug_assertions))]
        let options = serde_json::json!({
            "args": ["--window-size=1920,1080", "--headless"]
        });
        caps.insert("goog:chromeOptions".to_string(), options);
        let client = fantoccini::ClientBuilder::native()
            .capabilities(caps)
            .connect(&format!("http://localhost:{}", port))
            .await?;
        Ok(ChromeClient {
            client,
            wait_at_most,
        })
    }
}
