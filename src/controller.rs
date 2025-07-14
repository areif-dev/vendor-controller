use ean13::Ean13;

use crate::Product;

pub trait VendorController {
    fn login(&self) -> impl std::future::Future<Output = Result<(), fantoccini::error::CmdError>>;

    fn product_from_ean(
        &self,
        ean: Ean13,
    ) -> impl std::future::Future<Output = Result<Product, fantoccini::error::CmdError>>;
}
