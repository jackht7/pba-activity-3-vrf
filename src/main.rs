use lazy_static::lazy_static;
use player::Player;

pub mod player;

const PLAYER_COUNT: usize = 4;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

lazy_static! {
    static ref CARDS: Vec<Card> = vec![
        Card::Two,
        Card::Three,
        Card::Four,
        Card::Five,
        Card::Six,
        Card::Seven,
        Card::Eight,
        Card::Nine,
        Card::Ten,
        Card::Jack,
        Card::Queen,
        Card::King,
        Card::Ace,
    ];
}

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
fn main() {
    let players = (0..PLAYER_COUNT)
        .into_iter()
        .map(|_| Player::new())
        .collect::<Vec<_>>();

    loop {
        // game
        //
        // deal
        //
        // bid
    }
}
