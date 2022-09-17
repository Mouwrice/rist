use crate::boards::Board;
use crate::players::{Player, PlayerStruct};
use std::fmt::{Display, Formatter};

pub struct RandomPlayer {
    player: PlayerStruct,
}

impl Display for RandomPlayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.player.fmt(f)
    }
}

impl Player for RandomPlayer {
    fn claim_territory(&self, _board: &dyn Board) {
        todo!()
    }

    fn place_armies() {
        todo!()
    }

    fn attack() {
        todo!()
    }

    fn capture() {
        todo!()
    }

    fn defend() {
        todo!()
    }

    fn free_move() {
        todo!()
    }

    fn colorize(_player: &PlayerStruct, _text: String) {
        todo!()
    }
}
