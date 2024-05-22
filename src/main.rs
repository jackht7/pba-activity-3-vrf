pub mod player;

/**
 * player.rs
 * - Players need to have keypairs - their own secret and public keys.
 * - When a player or players draw a card, we need to choose an input for the VRFs of players who draw cards.
 * - Random number generation
 * - One good way to get an input is for all players to do a commit-reveal and combine the results,
 * however you could choose whatever technique you'd like.
 *
 * vrf.rs
 * - Players know their own VRF output (i.e. the cards in their hand),
 * but other players don't until the game calls for them to reveal their card, by publishing a VRF output.
 */
fn main() {}
