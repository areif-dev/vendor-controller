use gtin::Gtin;
use rust_decimal::{Decimal, prelude::Zero};
use serde::{Deserialize, Serialize};

/// Filters out any characters that aren't decimal digits or '-' or '.' and attempts to build a
/// [`Decimal`] from the remainder
///
/// # Arguments
/// - `raw` The unfiltered string to attempt converting into [`Decimal`]
///
/// # Returns
/// A [`Decimal`] parsed from the filtered string
///
/// # Errors
/// [`rust_decimal::Error`] if even the filtered string cannot be parsed into a [`Decimal`]
pub fn parse_price_nonstrict(raw: &str) -> Result<Decimal, rust_decimal::Error> {
    let s: String = raw
        .chars()
        .filter(|c| c.is_digit(10) || *c == '.' || *c == '-')
        .collect();
    Decimal::from_str_exact(&s)
}

/// Stores all pertinent details about a product listing from a vendor website including minimum
/// advertized price, manufacturer suggested retail, images, skus, descriptions, wholesales, and
/// GTIN code
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Product {
    gtin: Gtin,
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
    /// * `gtin` - 000000000000
    /// * `desc` - null string
    /// * `sku` - null string
    /// * `img_url` - "about:blank"
    /// * `wholesale` - $0.00
    /// * `msrp` - $0.00
    /// * `imap` - $0.00
    pub fn new() -> Self {
        Self {
            gtin: Gtin::nonstrict_new(""),
            desc: String::new(),
            sku: String::new(),
            img_url: String::from("about:blank"),
            wholesale: Decimal::zero(),
            msrp: Decimal::zero(),
            imap: Decimal::zero(),
        }
    }

    pub fn gtin(self, gtin: Gtin) -> Self {
        Self { gtin, ..self }
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

    pub fn get_gtin(&self) -> Gtin {
        self.gtin.clone()
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
