use std::{error::Error, str::FromStr};

use alloy::{
    hex,
    signers::{Signer, local::PrivateKeySigner},
};

pub struct Wallet;

pub struct Generate {
    private_key: String,
    address: String,
}

//generate wallet
impl Wallet {
    //generate new wallet evm return private_key and address
    pub fn generate() -> Generate {
        let walet = PrivateKeySigner::random();

        let private_key = hex::encode(walet.to_field_bytes());
        let address = walet.address().to_string();

        Generate {
            private_key,
            address,
        }
    }
}

pub struct SignMessage {
    wallet: Generate,
    signature: String,
}

//sign message
impl Wallet {
    pub async fn sign_message(
        wallet: Generate,
        message: String,
    ) -> Result<SignMessage, Box<dyn Error>> {
        let signer = PrivateKeySigner::from_str(&wallet.private_key)?;

        let sign = signer.sign_message(message.as_bytes()).await?;
        let signature = hex::encode(sign.as_bytes()); //experimental not sure is gonna work

        Ok(SignMessage { wallet, signature })
    }
}
