use colored::Color::{Magenta, White};
use std::rc::Rc;

use rist::dice;
use rist::players::random_player::RandomPlayer;
use rist::players::Player;

/// Every dice roll should be between 1 and 6
#[test]
fn test_roll_dice() {
    for roll in dice::roll_dice(10) {
        assert!((1..=6).contains(&roll));
    }
}

/// Visual check to see if the printed text is correct
#[test]
fn test_player_rolls_dice() {
    let player = RandomPlayer::new("TestPlayer", Magenta, White);
    assert!((1..=6).contains(&dice::player_rolls_dice(&player, 1, true)[0]));
    for roll in dice::player_rolls_dice(&player, 10, true) {
        assert!((1..=6).contains(&roll));
    }
}

/// Visual check to see if the printed text is correct
#[test]
fn test_players_roll_die() {
    let player1: Rc<dyn Player> = Rc::new(RandomPlayer::new("TestPlayer1", Magenta, White));
    let player2: Rc<dyn Player> = Rc::new(RandomPlayer::new("TestPlayer2", Magenta, White));
    for roll in dice::players_roll_die(&vec![&player1, &player2], true) {
        assert!((1..=6).contains(&roll));
    }
}
