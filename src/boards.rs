//! Defines the structure and traits for a board implementation.
//! The board provides important operations to play the game
//! such as claiming a territory and placing down armies on the board.
//! A board contains all territories and continents.
//!
//! This module provides default implementations that can be used if you so wish.
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use std::{io, thread};

use crate::continent::Continent;
use crate::players::Player;
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
    extra_info: RefCell<Vec<String>>,
    /// The maximum of extra info lines that are available
    /// Force prints the board when full and clears the extra lines afterwards
    extra_info_lines: usize,
    print_duration: Option<Duration>,
}

/// Provides a default implementation according to the standard ruleset
/// You are free to not use these functions and implement a different behaviour to your liking
impl BoardStruct {
    pub fn new(board: BoardType, players: usize, print_duration: Option<Duration>) -> BoardStruct {
        match board {
            BoardType::ClassicBoard => classic_board::new(players, print_duration),
            BoardType::Unimplemented => unimplemented!(),
        }
    }

    /// Generate a new `BoardStruct` and perform validity checks
    pub fn generate_board(
        board: BoardType,
        continents: Vec<&Rc<Continent>>,
        territories: Vec<&Rc<Territory>>,
        extra_info_lines: usize,
        print_duration: Option<Duration>,
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
            extra_info: RefCell::from(vec![]),
            extra_info_lines,
            print_duration,
        }
    }

    /// Default `claim_territory` implementation.
    /// Allows a player to claim a territory that is not yet occupied.
    /// Places 1 army on the board from the given player.
    /// `territory_index` points to the index of the territory in the `free_territories` list.
    /// Deletes the given territory from the `free_territories` list and sets the `player` in the `Territory`.
    pub fn claim_territory(
        &mut self,
        free_territory_index: usize,
        player: Rc<dyn Player>,
        verbose: bool,
    ) {
        // Territory lookup
        let territory_index = self.free_territories[free_territory_index];
        let territory = &self.territories[territory_index];

        let state = player.get_state();
        // The player needs to have one army available to claim a territory
        assert!(
            *state.armies.borrow() >= 1,
            "The player should have at least 1 army in it's inventory. {} has {} remaining",
            state.name,
            state.armies.borrow()
        );

        // The player cannot claim a territory that is already occupied
        assert!(
            territory.get_player().is_none(),
            "The territory is already occupied. {} tried to claim {}",
            state.name,
            territory.name
        );

        // Remove the territory from the free territories list
        self.free_territories.remove(free_territory_index);

        // Place army
        territory.place_armies(Rc::clone(&player), 1);

        // Assign territory to player
        territory.set_player(Some(Rc::downgrade(&player)));
        player.get_state().add_territory(Rc::clone(territory));

        if verbose {
            self.set_extra_info(format!(
                "{} claimed {}",
                player.get_state().name,
                territory.name
            ));
        }

        // Assign part of continent to player
        let continent_index = *territory.continent.index.borrow();
        let continent = &self.continents[continent_index];
        continent.territories_per_player.borrow_mut()[*player.get_state().index.borrow()] += 1;
        if continent.territories_per_player.borrow()[*player.get_state().index.borrow()]
            == continent.size
        {
            player.get_state().add_continent(Rc::clone(continent));

            if verbose {
                self.set_extra_info(format!(
                    "{} has claimed the entirety of {}",
                    player.get_state().name,
                    continent.name
                ));
            }
        }

        if verbose {
            self.print_board();
            self.clear_extra_info();
        }
    }

    /// Allows to add some extra text to the board representation
    pub fn set_extra_info(&self, text: String) {
        self.extra_info.borrow_mut().push(text);
        if self.extra_info.borrow().len() >= self.extra_info_lines {
            self.print_board();
            self.clear_extra_info();
        }
    }

    /// Clears the extra info
    pub fn clear_extra_info(&self) {
        self.extra_info.borrow_mut().clear();
    }

    /// Prints the board to stdout
    /// If `None` Duration is specified waits on `enter` input key to continue
    pub fn print_board(&self) {
        // Clears the terminal
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        match self.board {
            BoardType::ClassicBoard => classic_board::print_board(self),
            BoardType::Unimplemented => unimplemented!(),
        }
        if let Some(dur) = self.print_duration {
            thread::sleep(dur);
        } else {
            let mut buffer = String::new();
            let stdin = io::stdin(); // We get `Stdin` here.
            stdin
                .read_line(&mut buffer)
                .expect("Did not enter a valid string");
        }
    }
}
