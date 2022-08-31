//! Defines the structure and traits for a board implementation.
//! The board provides important operations to play the game
//! such as claiming a territory and placing down armies on the board.
//! A board contains all territories and continents.

use std::borrow::Borrow;
use std::rc::Rc;

use crate::continent::Continent;
use crate::player::PlayerStruct;
use crate::territory::Territory;
use crate::{continent, territory};

mod classic_board;

/// A general purpose Board struct
#[derive(Debug)]
pub struct BoardStruct<'a> {
    /// All continents of the entire map
    pub continents: Vec<Rc<Continent>>,
    /// All territories of the entire map
    pub territories: Vec<Rc<Territory<'a>>>,
    /// All territory IDs that are not yet claimed by a player
    pub free_territories: Vec<u32>,
}

impl<'a> BoardStruct<'a> {
    fn new_board_struct(
        continents: Vec<Rc<Continent>>,
        territories: Vec<Rc<Territory<'a>>>,
    ) -> BoardStruct<'a> {
        continent::generate_ids(
            continents
                .iter()
                .map(|continent| continent.borrow())
                .collect(),
        );
        territory::generate_ids(
            territories
                .iter()
                .map(|territory| territory.borrow())
                .collect(),
        );
        let free_territories = territories
            .iter()
            .map(|territory| *territory.id.borrow())
            .collect();

        BoardStruct {
            continents,
            territories,
            free_territories,
        }
    }
}

/// All boards should implements this trait
pub trait Board<'a> {
    /// Allows a player to claim a territory that is not yet claimed
    fn claim_territory(&self, territory_index: u32, player: &PlayerStruct) {}

    /// Place a given amount of armies on the specified territory for a player
    fn place_armies(&self, territory: &Territory, player: &PlayerStruct, armies: u32) {}
}
