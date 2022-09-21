use crate::Attack;
use itertools::enumerate;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
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
    let mut territories = vec![];
    let mut armies = vec![];

    let mut rng = thread_rng();
    let uniform = Uniform::new(0, *player.armies.borrow());

    let mut armies_placed = 0;
    for territory in player.territories.borrow().iter() {
        let amount = rng.sample(uniform);
        armies_placed += amount;

        if armies_placed > *player.armies.borrow() {
            break;
        }

        if amount > 0 {
            territories.push(Rc::clone(&board.territories[*territory.index.borrow()]));
            armies.push(amount);
        }
    }

    let mut placement = vec![];
    for (index, territory) in enumerate(territories) {
        placement.push((territory, armies[index]));
    }

    placement
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
