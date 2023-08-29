use colored::Color::{Blue, White};
use rist::players::{Player, RandomPlayer};
use std::rc::Rc;

/// Test string coloring

#[test]
fn test_colorize() {
    let player = Rc::new(RandomPlayer::new("TestPlayer", Blue, White));

    println!("{}", player.get_state().colorize(String::from("I am Blue")));
}
