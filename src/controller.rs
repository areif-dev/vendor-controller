use ean13::Ean13;
use fantoccini::{Locator, error::CmdError};
use rust_decimal::{Decimal, prelude::Zero};

use crate::{ChromeClient, Product};

pub trait VendorController {
    fn client(&self) -> &ChromeClient;

    fn login(&self) -> impl std::future::Future<Output = Result<(), fantoccini::error::CmdError>>;

    fn product_from_ean(
        &self,
        ean: Ean13,
    ) -> impl std::future::Future<Output = Result<Product, fantoccini::error::CmdError>>;
}
