use std::str::FromStr;

use crate::Wallet;
use alloy::signers::Signer;
use alloy::{hex, signers::local::PrivateKeySigner};
use anyhow::Result;

pub struct SignMessage {
    private_key: String,
    address: String,
    signature: String,
}

impl Wallet {
    pub async fn sign_message(
        private_key: &str,
        address: &str,
        message: String,
    ) -> Result<SignMessage> {
        let signer = PrivateKeySigner::from_str(private_key)?;

        let sign = signer.sign_message(message.as_bytes()).await?;
        let signature = hex::encode(sign.as_bytes()); //experimental not sure is gonna work

        Ok(SignMessage {
            signature,
            private_key: private_key.to_string(),
            address: address.to_string(),
        })
    }
}
