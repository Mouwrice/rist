use std::cmp::min;
use std::rc::Rc;

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

use crate::boards::BoardStruct;
use crate::players::PlayerStruct;
use crate::territory::Territory;
use crate::Attack;

pub fn claim_territory(board: &BoardStruct) -> usize {
    let distribution = Uniform::new_inclusive(0, &board.free_territories.len() - 1);
    let mut rng = thread_rng();
    rng.sample(distribution)
}

pub fn place_armies(player: &PlayerStruct, board: &BoardStruct) -> Vec<(Rc<Territory>, u32)> {
    let mut rng = thread_rng();
    let uniform_territories = Uniform::new(0, &player.territories.borrow().len() - 1);
    let mut armies_placed = 0;
    let mut placement = vec![];
    for _ in 0..rng.sample(uniform_territories) {
        let territory = &player.territories.borrow()[rng.sample(uniform_territories)];
        let uniform_armies = Uniform::new(0, *player.armies.borrow() - armies_placed);
        let armies = rng.sample(uniform_armies);
        armies_placed += armies;

        if armies > 0 {
            placement.push((
                Rc::clone(&board.territories[*territory.index.borrow()]),
                armies,
            ));
        }
    }
    placement
}

/// The random player attacks half of the times
pub fn attack(player: &PlayerStruct) -> Option<Attack> {
    let mut rng = thread_rng();
    if rng.gen::<f32>() < 0.5 {
        return None;
    }

    // Create all valid attacks
    let mut attacks = vec![];
    for territory in &*player.territories.borrow() {
        // An attacker should have at least 2 armies
        if *territory.armies.borrow() >= 2 {
            for adjacent_territory in &*territory.connections.borrow() {
                // Can only attack from a territory adjacent to an enemy territory
                if *adjacent_territory.upgrade().unwrap().get_player().unwrap() != *player {
                    let dice = Uniform::new(1, min(4, *territory.armies.borrow()));
                    attacks.push(Attack {
                        dice: rng.sample(dice),
                        attacker: Rc::clone(territory),
                        defender: Rc::clone(&adjacent_territory.upgrade().unwrap()),
                    })
                }
            }
        }
    }

    // Pick a random attack from the valid attacks
    if !attacks.is_empty() {
        let dist = Uniform::new(0, attacks.len());
        let attack = &attacks[rng.sample(dist)];
        return Some(Attack {
            dice: attack.dice,
            attacker: Rc::clone(&attack.attacker),
            defender: Rc::clone(&attack.defender),
        });
    }
    None
}

pub fn capture(attack: &Attack) -> u32 {
    let mut rng = thread_rng();
    let uniform = Uniform::new(
        min(attack.dice, *attack.attacker.armies.borrow()),
        *attack.attacker.armies.borrow(),
    );
    rng.sample(uniform)
}

pub fn defend(attack: &Attack) -> u32 {
    if *attack.defender.armies.borrow() == 1 {
        return 1;
    }

    let mut rng = thread_rng();
    let uniform = Uniform::new(1, 3);
    rng.sample(uniform)
}

pub fn free_move(_player: &PlayerStruct) {}
