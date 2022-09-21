use crate::Attack;
use rand::distributions::Uniform;
use rand::Rng;
use std::rc::Rc;

use crate::boards::BoardStruct;
use crate::players::PlayerStruct;
use crate::territory::Territory;

pub fn claim_territory(board: &BoardStruct) -> usize {
    let distribution = Uniform::new_inclusive(0, &board.free_territories.len() - 1);
    let mut rng = rand::thread_rng();
    rng.sample(distribution)
}

pub fn place_armies(player: &PlayerStruct, board: &BoardStruct) -> Vec<(Rc<Territory>, u32)> {
    vec![]
}

pub fn attack(player: &PlayerStruct, board: &BoardStruct) -> Option<Attack> {
    None
}

pub fn capture(player: &PlayerStruct) -> u32 {
    1
}

pub fn defend(player: &PlayerStruct) -> u32 {
    1
}

pub fn free_move(player: &PlayerStruct) {
    todo!()
}
