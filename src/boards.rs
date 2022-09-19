//! Defines the structure and traits for a board implementation.
//! The board provides important operations to play the game
//! such as claiming a territory and placing down armies on the board.
//! A board contains all territories and continents.
//!
//! This module provides default implementations that can be used if you so wish.
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::continent::Continent;
use crate::players::PlayerStruct;
use crate::territory::Territory;
use crate::{continent, territory};

pub mod classic_board;

#[derive(Debug)]
pub enum BoardType {
    Unimplemented,
    ClassicBoard,
}

/// The default internal structure of a board
#[derive(Debug)]
pub struct BoardStruct {
    pub board: BoardType,
    /// All continents of the entire map
    pub continents: Vec<Rc<Continent>>,
    /// All territories of the entire map
    pub territories: Vec<Rc<Territory>>,
    /// All territory IDs that are not yet claimed by a player
    pub free_territories: Vec<usize>,
}

/// Provides a default implementation according to the standard ruleset
/// You are free to not use these functions and implement a different behaviour to your liking
impl BoardStruct {
    pub fn new(board: BoardType, players: usize) -> BoardStruct {
        match board {
            BoardType::ClassicBoard => classic_board::new(players),
            BoardType::Unimplemented => unimplemented!(),
        }
    }

    /// Generate a new `BoardStruct` and perform validity checks
    pub fn generate_board(
        board: BoardType,
        continents: Vec<&Rc<Continent>>,
        territories: Vec<&Rc<Territory>>,
    ) -> BoardStruct {
        continent::generate_ids(&continents);
        territory::generate_ids(&territories);
        let free_territories = territories
            .iter()
            .map(|territory| *territory.index.borrow())
            .collect::<Vec<usize>>();

        BoardStruct {
            board,
            continents: continents
                .iter()
                .map(|continent| Rc::clone(continent))
                .collect(),
            territories: territories
                .iter()
                .map(|territory| Rc::clone(territory))
                .collect(),
            free_territories,
        }
    }

    /// Default `claim_territory` implementation.
    /// Allows a player to claim a territory that is not yet occupied.
    /// Places 1 army on the board from the given player.
    /// `territory_index` points to the index of the territory in the `free_territories` list.
    /// Deletes the given territory from the `free_territories` list and sets the `player` in the `Territory`.
    pub fn claim_territory(&mut self, free_territory_index: usize, player: &Rc<PlayerStruct>) {
        // Territory lookup
        let territory_index = self.free_territories[free_territory_index];
        self.free_territories.remove(free_territory_index);
        let territory = &self.territories[territory_index];

        if *player.armies.borrow() < 1 {
            panic!(
                "The player should have at least 1 army in it's inventory. {} has {} remaining",
                player.name,
                player.armies.borrow()
            );
        }

        // Place army
        territory.place_armies(player, 1);

        // Assign territory to player
        *territory.player.borrow_mut() = Some(Rc::downgrade(player));
        player.territories.borrow_mut().insert(Rc::clone(territory));

        // Assign part of continent to player
        let continent_index = *territory.continent.index.borrow();
        let continent = &self.continents[continent_index];
        continent.territories_per_player.borrow_mut()[*player.index.borrow()] += 1;
        if continent.territories_per_player.borrow()[*player.index.borrow()] == continent.size {
            player.continents.borrow_mut().insert(Rc::clone(continent));
        }
    }
}

impl Display for BoardStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.board {
            BoardType::ClassicBoard => classic_board::fmt(self, f),
            BoardType::Unimplemented => unimplemented!(),
        }
    }
}
