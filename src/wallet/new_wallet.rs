use crate::Wallet;
use alloy::{hex, signers::local::PrivateKeySigner};

pub struct NewWallet {
    pub private_key: String,
    pub address: String,
}

impl Wallet {
    pub fn new_wallet() -> NewWallet {
        let walet = PrivateKeySigner::random();

        let private_key = hex::encode(walet.to_field_bytes());
        let address = walet.address().to_string();

        NewWallet {
            private_key,
            address,
        }
    }
}
