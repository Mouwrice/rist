//! These functions below provide ways to roll dice
use itertools::{enumerate, join};
use rand::distributions::Uniform;
use rand::Rng;

use crate::player::PlayerStruct;

/// Simulates rolling a give amount of standard 6 sided dice
pub fn roll_dice(amount: u32) -> Vec<u32> {
    let dice = Uniform::new_inclusive(1, 6);
    let mut rng = rand::thread_rng();
    (0..amount).map(|_| rng.sample(dice)).collect()
}

/// Lets a player roll a given amount of standard 6 sided dice
pub fn player_rolls_dice(player: &PlayerStruct, amount: u32) -> Vec<u32> {
    let rolls = roll_dice(amount);
    let joined = join(&rolls, ", ");
    println!(
        "{} rolls {} {}: {}",
        player.name,
        amount,
        if amount > 1 { "dice" } else { "die" },
        joined
    );
    rolls
}

/// Rolls a standard 6 sided die for every player
/// The order of rolls is the same as the order of players that is given
pub fn players_roll_die(players: &Vec<&PlayerStruct>) -> Vec<u32> {
    let rolls = roll_dice(players.len() as u32);
    for (i, player) in enumerate(players) {
        println!("{} rolled {}", player.name, rolls[i]);
    }
    rolls
}
