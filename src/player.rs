use schnorrkel::{Keypair, PublicKey, SecretKey};

pub struct Player(Keypair);

impl Player {
    pub fn new() -> Self {
        Self(Keypair::generate())
    }

    pub fn keypair(&self) -> &Keypair {
        &self.0
    }

    pub fn key_private(&self) -> &SecretKey {
        &self.0.secret
    }

    pub fn key_public(&self) -> &PublicKey {
        &self.0.public
    }
}
