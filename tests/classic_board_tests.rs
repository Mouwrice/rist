use colored::Color::{Blue, Green, Red, White};
use rist::boards::classic_board;
use rist::players::PlayerStruct;
use rist::players::PlayerType::Unimplemented;
use std::rc::Rc;

/// Visual test to test the display of the board
#[test]
fn test_display() {
    let mut board = classic_board::new(4);
    let player1 = Rc::new(PlayerStruct::new(Unimplemented, "TestPlayer1", Blue, White));
    let player2 = Rc::new(PlayerStruct::new(
        Unimplemented,
        "TestPlayer2",
        Green,
        White,
    ));
    let player3 = Rc::new(PlayerStruct::new(Unimplemented, "TestPlayer3", Red, White));

    board.claim_territory(0, Rc::clone(&player1));
    board.claim_territory(0, Rc::clone(&player2));
    board.claim_territory(0, Rc::clone(&player2));
    board.claim_territory(0, Rc::clone(&player1));
    board.claim_territory(0, Rc::clone(&player3));
    board.claim_territory(0, Rc::clone(&player3));
    board.claim_territory(0, Rc::clone(&player2));
    board.claim_territory(0, Rc::clone(&player1));
    board.claim_territory(0, Rc::clone(&player1));
    board.claim_territory(0, Rc::clone(&player3));
    board.claim_territory(0, Rc::clone(&player1));

    println!("{}", board);
}
