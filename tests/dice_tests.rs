use colored::Color::{Magenta, White};

use rist::dice;
use rist::players::PlayerStruct;
use rist::players::PlayerType::RandomPlayer;

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
    let player = PlayerStruct::new(RandomPlayer, "TestPlayer", 0, Magenta, White);
    assert!((1..=6).contains(&dice::player_rolls_dice(&player, 1)[0]));
    for roll in dice::player_rolls_dice(&player, 10) {
        assert!((1..=6).contains(&roll));
    }
}

/// Visual check to see if the printed text is correct
#[test]
fn test_players_roll_die() {
    let player1 = PlayerStruct::new(RandomPlayer, "TestPlayer1", 0, Magenta, White);
    let player2 = PlayerStruct::new(RandomPlayer, "TestPlayer2", 0, Magenta, White);
    for roll in dice::players_roll_die(&vec![&player1, &player2]) {
        assert!((1..=6).contains(&roll));
    }
}
