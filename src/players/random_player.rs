use rand::distributions::Uniform;
use rand::Rng;

use crate::boards::BoardStruct;
use crate::players::PlayerStruct;

pub fn claim_territory(board: &BoardStruct) -> usize {
    let distribution = Uniform::new_inclusive(0, &board.free_territories.len() - 1);
    let mut rng = rand::thread_rng();
    rng.sample(distribution)
}

pub fn place_armies(player: &PlayerStruct) {
    todo!()
}

pub fn attack(player: &PlayerStruct) {
    todo!()
}

pub fn capture(player: &PlayerStruct) {
    todo!()
}

pub fn defend(player: &PlayerStruct) {
    todo!()
}

pub fn free_move(player: &PlayerStruct) {
    todo!()
}
