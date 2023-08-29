use colored::Color::{Black, Blue, Green, Red, White};
use rist::boards::BoardStruct;
use rist::boards::BoardType::ClassicBoard;
use rist::players::random_player::RandomPlayer;
use rist::players::Player;
use rist::Game;
use std::rc::Rc;

fn main() {
    let players: Vec<Rc<dyn Player>> = vec![
        Rc::new(RandomPlayer::new("Player 1", Red, White)),
        Rc::new(RandomPlayer::new("Player 2", Green, White)),
        Rc::new(RandomPlayer::new("Player 3", Blue, White)),
        Rc::new(RandomPlayer::new("Player 4", White, Black)),
    ];

    let board = BoardStruct::new(ClassicBoard, players.len(), None);
    let mut game = Game::new(players, board);
    game.setup(true);

    game.play(None, Some(100), false, true);
}
