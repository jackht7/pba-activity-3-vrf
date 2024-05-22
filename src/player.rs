use std::str::from_utf8;

use schnorrkel::{
    context::signing_context,
    vrf::VRFProof,
    Keypair, PublicKey, SecretKey,
};

pub struct Player {
    key_pair: Keypair,
    hand: Option<u32>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            key_pair: Keypair::generate(),
            hand: None,
        }
    }

    pub fn key_private(&self) -> &SecretKey {
        &self.key_pair.secret
    }

    pub fn key_public(&self) -> &PublicKey {
        &self.key_pair.public
    }

    pub fn draw(&mut self, input: &[u8]) -> VRFProof {
        let ctx = signing_context(b"yo!");
        let (output, signature, _) = self.key_pair.vrf_sign(ctx.bytes(input));

        let output_byte = output.as_output_bytes();
        let hand_index = from_utf8(output_byte).unwrap().parse::<u32>().unwrap() % 13;

        self.hand = Some(hand_index);

        signature
    }
}
