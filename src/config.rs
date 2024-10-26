use doli_client_api_rs::{error::DoliApiClientError, Client as ClientDoli};
use get_pass::get_password;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    // cover database connection
    pub db_uri: Url,
    pub db_pass_path: PathBuf,
    // port on which the cover API will listen for incoming connections
    pub listen_port: u16,
    // cover database connection
    pub product_api_uri: Url,
    pub product_api_pass_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_uri: Url::parse("postgresql:://user@127.0.0.1:5432/mydb").unwrap(),
            db_pass_path: PathBuf::from("name_api/db/user"),
            listen_port: 10200,
            product_api_uri: Url::parse("https://ecommerce.com/product-api").unwrap(),
            product_api_pass_path: PathBuf::from("product-api/user"),
        }
    }
}

impl Config {
    pub fn product_client(&self) -> anyhow::Result<ClientDoli> {
        let mut url = self.product_api_uri.clone();
        url.set_password(get_password(&self.product_api_pass_path).ok().as_deref())
            .map_err(|_| DoliApiClientError::InvalidToken)?;
        Ok(ClientDoli::new(url)?)
    }
}
