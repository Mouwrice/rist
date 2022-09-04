use colored::Color::{Blue, Green, Red, White};
use rist::boards::{classic_board, Board};
use rist::player::PlayerStruct;
use std::rc::Rc;

/// Visual test to test the display of the board
#[test]
fn test_display() {
    let mut board = classic_board::new(4);
    let player1 = Rc::new(PlayerStruct::new("TestPlayer1", 10, Blue, White));
    let player2 = Rc::new(PlayerStruct::new("TestPlayer2", 3, Green, White));
    let player3 = Rc::new(PlayerStruct::new("TestPlayer3", 5, Red, White));

    board.claim_territory(0, &player1);
    board.claim_territory(0, &player2);
    board.claim_territory(0, &player2);
    board.claim_territory(0, &player1);
    board.claim_territory(0, &player3);
    board.claim_territory(0, &player3);
    board.claim_territory(0, &player2);
    board.claim_territory(0, &player1);
    board.claim_territory(0, &player1);
    board.claim_territory(0, &player3);
    board.claim_territory(0, &player1);

    println!("{}", board);
}
