use ean13::Ean13;
use rust_decimal::{Decimal, prelude::Zero};
use serde::{Deserialize, Serialize};

/// Stores all pertinent details about a product listing from a vendor website including minimum
/// advertized price, manufacturer suggested retail, images, skus, descriptions, wholesales, and
/// EAN-13 code
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    ean13: Ean13,
    desc: String,
    sku: String,
    wholesale: Decimal,
    img_url: String,
    msrp: Decimal,
    imap: Decimal,
}

impl Default for Product {
    fn default() -> Self {
        Product::new()
    }
}

impl Product {
    /// Create a simple [`Product`] with sensible defaults. Specific values will be
    /// * `ean13` - 000000000000
    /// * `desc` - null string
    /// * `sku` - null string
    /// * `img_url` - "about:blank"
    /// * `wholesale` - $0.00
    /// * `msrp` - $0.00
    /// * `imap` - $0.00
    pub fn new() -> Self {
        Self {
            ean13: Ean13::default(),
            desc: String::new(),
            sku: String::new(),
            img_url: String::from("about:blank"),
            wholesale: Decimal::zero(),
            msrp: Decimal::zero(),
            imap: Decimal::zero(),
        }
    }

    pub fn ean13(self, ean13: Ean13) -> Self {
        Self { ean13, ..self }
    }

    pub fn desc<S>(self, desc: S) -> Self
    where
        S: ToString,
    {
        Self {
            desc: desc.to_string(),
            ..self
        }
    }

    pub fn sku<S>(self, sku: S) -> Self
    where
        S: ToString,
    {
        Self {
            sku: sku.to_string(),
            ..self
        }
    }

    pub fn img_url<S>(self, img_url: S) -> Self
    where
        S: ToString,
    {
        Self {
            img_url: img_url.to_string(),
            ..self
        }
    }

    pub fn msrp(self, msrp: Decimal) -> Self {
        Self { msrp, ..self }
    }

    pub fn imap(self, imap: Decimal) -> Self {
        Self { imap, ..self }
    }

    pub fn wholesale(self, wholesale: Decimal) -> Self {
        Self { wholesale, ..self }
    }

    pub fn get_ean13(&self) -> Ean13 {
        self.ean13.clone()
    }

    pub fn get_desc(&self) -> String {
        self.desc.clone()
    }

    pub fn get_sku(&self) -> String {
        self.sku.clone()
    }

    pub fn get_img_url(&self) -> String {
        self.img_url.clone()
    }

    pub fn get_msrp(&self) -> Decimal {
        self.msrp
    }

    pub fn get_imap(&self) -> Decimal {
        self.imap
    }

    pub fn get_wholesale(&self) -> Decimal {
        self.wholesale
    }
}
