use solana_sdk::signature::{Keypair};

pub struct Account {
    pub keypair: Keypair,
}

pub trait FromBase58 {
    fn from_base58_str(base58_str: &str) -> Self;
}

impl FromBase58 for Account {
    fn from_base58_str(base58_str: &str) -> Self {
        Account {
            keypair: Keypair::from_base58_string(base58_str)
        }
    }
}