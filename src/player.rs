use schnorrkel::{PublicKey, SecretKey};

pub struct Player {
    key_private: SecretKey,
    key_public: PublicKey,
}

impl Player {
    pub fn new() -> Self {
        let key_private = SecretKey::generate();
        let key_public = PublicKey::from(key_private.clone());

        Self {
            key_private,
            key_public,
        }
    }

    pub fn key_private(&self) -> &SecretKey {
        &self.key_private
    }

    pub fn key_public(&self) -> &PublicKey {
        &self.key_public
    }
}
