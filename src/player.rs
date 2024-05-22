use schnorrkel::{Keypair, PublicKey, SecretKey};

pub const DEFAULT_BALANCE: u32 = 100;

pub struct Player {
    keys: Keypair,
    pub balance: u32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            keys: Keypair::generate(),
            balance: DEFAULT_BALANCE,
        }
    }

    pub fn keypair(&self) -> &Keypair {
        &self.keys
    }

    pub fn key_private(&self) -> &SecretKey {
        &self.keys.secret
    }

    pub fn key_public(&self) -> &PublicKey {
        &self.keys.public
    }
}
