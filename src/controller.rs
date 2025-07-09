use ean13::Ean13;

use crate::Product;

pub trait VendorController {
    fn search<S>(&self, query: S) -> Vec<Product>
    where
        S: ToString;

    fn fetch_details(&self, ean: Ean13);
}
