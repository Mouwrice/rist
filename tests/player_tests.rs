use colored::Color::{Blue, White};
use rist::players::PlayerStruct;
use std::rc::Rc;

/// Test string coloring

#[test]
fn test_colorize() {
    let player = Rc::new(PlayerStruct::new("TestPlayer", 5, Blue, White));

    println!("{}", player.colorize(String::from("I am Blue")));
}
