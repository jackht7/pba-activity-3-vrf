use std::cmp::Ordering;

use lazy_static::lazy_static;
use player::Player;
use rand::Rng;
use schnorrkel::signing_context;

pub mod player;

const PLAYER_COUNT: usize = 4;

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
    let mut players = (0..PLAYER_COUNT)
        .into_iter()
        .map(|_| Player::new())
        .collect::<Vec<_>>();

    loop {
        // draws cards for each player
        let mut rng = rand::thread_rng();
        players.iter_mut().for_each(|player| {
            let seed = rng.gen::<u32>().to_be_bytes();
            player.draw(&seed);
        });

        let bets: Vec<_> = players
            .iter()
            .map(|player| rng.gen_range(0..player.balance))
            .collect();

        // find the winner
        // TODO: we don't need to do this if we save the highest generated card
        let winners = players
            .iter()
            .fold(Vec::<Player>::default(), |mut winners, player| {
                let winner = winners.first().map(|winner| winner.hand).unwrap_or(Some(0));
                match player.hand.cmp(&winner) {
                    Ordering::Less => winners,
                    Ordering::Equal => {
                        winners.push(player.clone());
                        winners
                    }
                    Ordering::Greater => vec![player.clone()],
                }
            });
    }
}
