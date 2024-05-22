use std::str::from_utf8;

use merlin::Transcript;
use schnorrkel::{
    context::signing_context,
    errors::SignatureError,
    vrf::{VRFInOut, VRFProof, VRFSigningTranscript},
    Keypair, PublicKey, SecretKey,
};

#[derive(Clone)]
pub struct Player(Keypair);

fn as_u32_be(array: &[u8; 32]) -> u32 {
    ((array[0] as u32) << 24)
        + ((array[1] as u32) << 16)
        + ((array[2] as u32) << 8)
        + ((array[3] as u32) << 0)
}

impl Player {
    pub fn new() -> Self {
        Self(Keypair::generate())
    }

    pub fn keys(&self) -> &Keypair {
        &self.0
    }

    pub fn draw(&self, input: &[u8]) -> (u32, Transcript, VRFInOut, VRFProof) {
        let ctx = signing_context(b"Drawing card");
        let transcript = ctx.bytes(input);
        let (output, signature, _) = self.0.vrf_sign(transcript.clone());

        let output_byte = output.as_output_bytes();
        let card = as_u32_be(output_byte) % 13;

        (card, transcript, output, signature)
    }

    pub fn validate(&self) {}
}
