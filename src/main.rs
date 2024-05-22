use std::cmp::Ordering;

use itertools::{multizip, MultiUnzip};
use player::Player;
use rand::Rng;

pub mod player;

const PLAYER_COUNT: usize = 4;
const DEFAULT_BALANCE: u32 = 100;

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

    let mut balances = (0..PLAYER_COUNT)
        .into_iter()
        .map(|_| DEFAULT_BALANCE)
        .collect::<Vec<_>>();

    let mut rng = rand::thread_rng();

    loop {
        for i in 0..PLAYER_COUNT {
            println!(
                "{:?} - player: {:?} - balance: {:?}",
                i,
                players[i].keys().public,
                balances[i]
            );
        }

        // Each player locks a random bet in advance
        let bets = balances
            .iter()
            .map(|balance| rng.gen_range(0..*balance + 1))
            .collect::<Vec<_>>();

        // Each player draws a verifiably random card
        let (cards, transcripts, out, proofs): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) =
            players
                .iter()
                .map(|player| {
                    let seed: u32 = rng.gen();
                    player.draw(&seed.to_be_bytes())
                })
                .multiunzip();
        let zipped = multizip((transcripts, out, proofs));

        // Checks each vfr
        let verify = (0..PLAYER_COUNT)
            .into_iter()
            .map(|player| {
                zipped
                    .clone()
                    .enumerate()
                    .filter(|(i, (_, _, _))| *i != player)
                    .map(|(i, (transcipt, out, proof))| {
                        players[i]
                            .keys()
                            .public
                            .vrf_verify(transcipt, &out.to_preout(), &proof)
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
            .is_ok();

        if verify == false {
            return;
        }

        // Sccumulates winners and losers, taking ties into consideration.
        // Players are identified by index.
        let (winners, loosers) =
            cards.into_iter().enumerate().fold(
                (
                    Vec::<(usize, u32)>::default(),
                    Vec::<(usize, u32)>::default(),
                ),
                |(mut winners, mut loosers), (i, player_hand)| {
                    let winner = winners.first().cloned().unwrap_or_default();
                    match player_hand.cmp(&winner.1) {
                        Ordering::Less => {
                            loosers.push((i, player_hand));
                            (winners, loosers)
                        }
                        Ordering::Equal => {
                            winners.push((i, player_hand));
                            (winners, loosers)
                        }
                        Ordering::Greater => {
                            loosers.append(&mut winners);
                            (vec![(i, player_hand)], loosers)
                        }
                    }
                },
            );

        // Determines the wins per winner, deducing losses from each looser balance.
        let wins: u32 = loosers.into_iter().fold(0, |wins, (i, _)| {
            balances[i] -= bets[i];
            wins + bets[i]
        }) / winners.len() as u32;

        // Adds wins to each winner's balance.
        winners.into_iter().for_each(|(i, _)| balances[i] += wins);

        // check for end of game
        if balances.iter().min().cloned().unwrap_or_default() == 0 {
            break;
        }
    }
}
