use colored::Color::{Blue, White};
use rist::players::PlayerStruct;
use rist::players::PlayerType::Unimplemented;
use std::rc::Rc;

/// Test string coloring

#[test]
fn test_colorize() {
    let player = Rc::new(PlayerStruct::new(Unimplemented, "TestPlayer", Blue, White));

    println!("{}", player.colorize(String::from("I am Blue")));
}
