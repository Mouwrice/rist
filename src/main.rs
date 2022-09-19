use colored::Color::{Blue, Cyan, Green, Red, White};
use rist::boards::BoardStruct;
use rist::boards::BoardType::ClassicBoard;
use rist::players::PlayerStruct;
use rist::players::PlayerType::RandomPlayer;
use rist::Game;
use std::rc::Rc;

fn main() {
    let players = vec![
        Rc::new(PlayerStruct::new(RandomPlayer, "Player 1", Red, White)),
        Rc::new(PlayerStruct::new(RandomPlayer, "Player 2", Green, White)),
        Rc::new(PlayerStruct::new(RandomPlayer, "Player 3", Blue, White)),
        Rc::new(PlayerStruct::new(RandomPlayer, "Player 4", Cyan, White)),
    ];

    let board = BoardStruct::new(ClassicBoard, players.len());
    let mut game = Game::new(players, board);
    game.setup();
}
