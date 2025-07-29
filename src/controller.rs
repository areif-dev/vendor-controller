use ean13::Ean13;
use fantoccini::{Locator, error::CmdError};
use rust_decimal::Decimal;

use crate::{ChromeClient, Product};

pub trait VendorController {
    /// Get a shared reference to this [`VendorController`]'s [`ChromeClient`]. This is useful for
    /// writing custom actions that need to interact with the underlying chromedriver
    fn client(&self) -> &ChromeClient;

    /// The default URL where this vendor's login UI exists. For example
    /// `String::from("https://www.vendor.com")`
    fn base_url() -> String;

    /// Fetch the username for logging into this vendor
    fn get_user(&self) -> String;

    /// Fetch the password for logging into this vendor
    fn get_passwd(&self) -> String;

    /// Navigate this vendor's website to login to your account. A default implementation is
    /// provided that attempts to look for username and password fields on the page associated with
    /// [`VendorController::base_url`]
    ///
    /// # Returns
    ///
    /// Unit type is returned if successful
    ///
    /// # Errors
    ///
    /// Forwards any [`fantoccini::error::CmdError`] that arises during the login attempt
    fn login(
        &self,
    ) -> impl std::future::Future<Output = Result<(), fantoccini::error::CmdError>> + Send
    where
        Self: Sync,
    {
        async {
            self.client().client.goto(&Self::base_url()).await?;
            let login_form_elem = self.client().client.find(Locator::Css("form")).await?;
            let inputs = login_form_elem.find_all(Locator::Css("input")).await?;
            let (mut user_input, mut passwd_input) = (None, None);
            for input in inputs {
                if user_input.is_some() && passwd_input.is_some() {
                    break;
                }
                let name = input
                    .prop("name")
                    .await?
                    .unwrap_or(String::new())
                    .to_lowercase();
                if name.contains("user") {
                    user_input = Some(input);
                } else if name.contains("pass") {
                    passwd_input = Some(input);
                }
            }
            let user_input = user_input.ok_or(fantoccini::error::CmdError::InvalidArgument(
                String::from("user_input"),
                String::from("Could not find a valid username input field on the page"),
            ))?;
            let passwd_input = passwd_input.ok_or(fantoccini::error::CmdError::InvalidArgument(
                String::from("passwd_input"),
                String::from("Could not find a valid password input field on the page"),
            ))?;
            user_input.send_keys(&self.get_user()).await?;
            passwd_input.send_keys(&self.get_passwd()).await?;
            passwd_input.send_keys("\n").await?;
            Ok(())
        }
    }

    /// Navigate the vendor's online catalog to fetch product information for a particular [`Ean13`] or UPC.
    ///
    /// # Arguments
    ///
    /// * `ean` - The unique barcode/upc/ean-13 that belongs to the product to search for
    ///
    /// # Returns
    ///
    /// If no errors occur, and the product exists in the catalog, then returns Some([`Product`]).
    /// If the product is not in the catalog, or is not identified by the specified [`Ean13`],
    /// then returns `None`
    ///
    /// # Errors
    ///
    /// Forwards any [`fantoccini::error::CmdError`]s that arise
    fn product_from_ean(
        &self,
        ean: Ean13,
    ) -> impl std::future::Future<Output = Result<Option<Product>, fantoccini::error::CmdError>>;

    /// Attempt to read the text of an element on screen and parse a [`Decimal`] value from it.
    ///
    /// # Arguments
    ///
    /// * `search` - The [`Locator`] to use that selects the element to parse.
    ///
    /// # Returns
    ///
    /// If the searched element contains a parsible [`Decimal`], returns Some([`Decimal`]). If the
    /// element does not contain a parsible [`Decimal`], returns `None`.
    ///
    /// # Errors
    ///
    /// Forwards any [`fantoccini::error::CmdError`]s that arise
    fn price_from_elem<'a>(
        &self,
        search: Locator<'a>,
    ) -> impl std::future::Future<Output = Result<Option<Decimal>, CmdError>> + Send
    where
        Self: Sync,
    {
        async move {
            let price_elem = self.client().client.find(search).await?;
            let mut price_str: String = price_elem.prop("value").await?.unwrap_or(String::new());
            if price_str == String::new() {
                price_str = price_elem.text().await.unwrap_or(String::new());
            }
            price_str = price_str
                .chars()
                .filter(|c| c.is_digit(10) || *c == '.')
                .collect();
            let Ok(val) = Decimal::from_str_exact(&price_str) else {
                return Ok(None);
            };
            Ok(Some(val))
        }
    }
}
