use crate::boards::BoardStruct;
use crate::players::PlayerState;
use crate::territory::Territory;
use crate::Attack;
use itertools::enumerate;
use std::rc::Rc;

/// Implement this trait to create your own player
pub trait Player {
    /// Get the internal state of the player
    fn get_state(&self) -> &PlayerState;

    /// Allows a player to claim a territory that is not yet claimed
    /// Returns the index of the free territory
    fn claim_territory(&self, board: &BoardStruct) -> usize;

    /// Allows the player to place armies on owned territories
    /// Returns a list of tuples containing the territories to place troops on
    fn place_armies(&self, board: &BoardStruct) -> Vec<(Rc<Territory>, u32)>;

    /// Gives the player an option to attack
    /// To end the attacking phase the player returns `None`
    fn attack(&self, board: &BoardStruct) -> Option<Attack>;

    /// When a player takes a territory it must assign a number of armies to that territory
    /// no less than the number of dice rolled
    fn capture(&self, attack: &Attack) -> u32;

    /// Called when the player is being attacked
    /// The player must return with how many dice it wishes to defend its territory
    fn defend(&self, attack: &Attack) -> u32;
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerType {
    Unimplemented,
    RandomPlayer,
}

/// Generated ids for a list of all players. We use the player index as ID
pub fn generate_ids(players: &Vec<Rc<dyn Player>>) {
    for (i, player) in enumerate(players) {
        *player.get_state().index.borrow_mut() = i;
    }
}
