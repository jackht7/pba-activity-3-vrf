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
        let (output, signature, _) = self.0.vrf_sign(transcript);

        let output_byte = output.as_output_bytes();
        let card = from_utf8(output_byte).unwrap().parse::<u32>().unwrap() % 13;

        (card, transcript, output, signature)
    }

    pub fn validate(&self) {}
}
