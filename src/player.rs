use schnorrkel::{
    context::signing_context,
    vrf::{VRFInOut, VRFProof},
    Keypair, PublicKey, SecretKey,
};

pub struct Player {
    key_pair: Keypair,
    hand: Option<VRFInOut>,
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

        self.hand = Some(output);

        signature
    }
}
