use colored::Color::{Blue, White};
use rist::player::{colorize, PlayerStruct};
use std::rc::Rc;

/// Test string coloring

#[test]
fn test_colorize() {
    let player = Rc::new(PlayerStruct::new("TestPlayer", 5, Blue, White));

    println!("{}", colorize(&player, String::from("I am Blue")));
}
