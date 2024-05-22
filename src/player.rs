use std::str::from_utf8;

use schnorrkel::{context::signing_context, errors::SignatureError, Keypair, PublicKey, SecretKey};

#[derive(Clone)]
pub struct Player(Keypair);

impl Player {
    pub fn new() -> Self {
        Self(Keypair::generate())
    }

    pub fn draw(&self, input: &[u8]) -> (u32, VRFProof) {
        let ctx = signing_context(b"Drawing card");
        let (output, signature, _) = self.0.vrf_sign(ctx.bytes(input));

        let output_byte = output.as_output_bytes();
        let card = from_utf8(output_byte).unwrap().parse::<u32>().unwrap() % 13;

        (card, signature)
    }

    pub fn validate(&self) {}
}
