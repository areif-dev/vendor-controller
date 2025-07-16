use ean13::Ean13;
use fantoccini::{Locator, error::CmdError};
use rust_decimal::{Decimal, prelude::Zero};

use crate::{ChromeClient, Product};

pub trait VendorController {
    fn client(&self) -> &ChromeClient;

    fn base_url() -> String;

    fn get_user(&self) -> String;

    fn get_passwd(&self) -> String;

    async fn login(&self) -> Result<(), fantoccini::error::CmdError> {
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

    fn product_from_ean(
        &self,
        ean: Ean13,
    ) -> impl std::future::Future<Output = Result<Product, fantoccini::error::CmdError>>;
}
