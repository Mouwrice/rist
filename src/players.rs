//! Defines the player trait and a default player structure that can be used
//! A player is not allowed to directly mutate the board struct

use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

use crate::Attack;
use colored::{Color, ColoredString, Colorize};
use itertools::enumerate;

use crate::boards::BoardStruct;
use crate::continent::Continent;
use crate::territory::Territory;

pub mod random_player;

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerType {
    Unimplemented,
    RandomPlayer,
}

/// The default internal structure of a player
/// `Player` gets used with an `Rc` and can therefore only have mutable fields with a `RefCell`
#[derive(Debug, PartialEq, Eq)]
pub struct PlayerStruct {
    pub player: PlayerType,
    pub index: RefCell<usize>,
    pub name: String,
    pub armies: RefCell<u32>,
    pub territories: RefCell<HashSet<Rc<Territory>>>,
    pub continents: RefCell<HashSet<Rc<Continent>>>,
    foreground: Color,
    background: Color,
    pub defeated: RefCell<bool>,
}

impl PlayerStruct {
    pub fn new(player: PlayerType, name: &str, background: Color, foreground: Color) -> Self {
        PlayerStruct {
            player,
            index: RefCell::from(0),
            name: String::from(name),
            armies: RefCell::from(0),
            territories: RefCell::from(HashSet::new()),
            continents: RefCell::from(HashSet::new()),
            foreground,
            background,
            defeated: RefCell::from(false),
        }
    }

    /// Color the text to the color of the player
    pub fn colorize(&self, text: String) -> ColoredString {
        text.color(self.foreground).on_color(self.background)
    }

    /// Allows a player to claim a territory that is not yet claimed
    /// Returns the index of the free territory
    pub fn claim_territory(&self, board: &BoardStruct) -> usize {
        match &self.player {
            PlayerType::RandomPlayer => random_player::claim_territory(board),
            PlayerType::Unimplemented => unimplemented!(),
        }
    }

    /// Allows the player to place armies on owned territories
    /// Returns a list of tuples containing the territories to place troops on
    pub fn place_armies(&self, board: &BoardStruct) -> Vec<(Rc<Territory>, u32)> {
        match &self.player {
            PlayerType::RandomPlayer => random_player::place_armies(self, board),
            PlayerType::Unimplemented => unimplemented!(),
        }
    }

    /// Gives the player an option to attack
    /// To end the attacking phase the player returns `None`
    pub fn attack(&self, board: &BoardStruct) -> Option<Attack> {
        match &self.player {
            PlayerType::RandomPlayer => random_player::attack(self, board),
            PlayerType::Unimplemented => unimplemented!(),
        }
    }

    /// When a player takes a territory it must assign a number of armies to that territory
    pub fn capture(&self, _attack: &Attack) -> u32 {
        match &self.player {
            PlayerType::RandomPlayer => random_player::capture(self),
            PlayerType::Unimplemented => unimplemented!(),
        }
    }

    /// Called when the player is being attacked
    /// The player must return with how many dice it wishes to defend its territory
    pub fn defend(&self, _attack: &Attack) -> u32 {
        match &self.player {
            PlayerType::RandomPlayer => random_player::defend(self),
            PlayerType::Unimplemented => unimplemented!(),
        }
    }

    fn free_move(&self) {
        match &self.player {
            PlayerType::RandomPlayer => random_player::free_move(self),
            PlayerType::Unimplemented => unimplemented!(),
        }
    }
}

impl Display for PlayerStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n\
            \tplayer: {:?}\n\
            \tindex: {}\n\
            \tarmies: {}\n\
            \tterritories: {}\n\
            \tcontinents: {}",
            self.name,
            self.player,
            self.index.borrow(),
            self.armies.borrow(),
            self.territories
                .borrow()
                .iter()
                .map(|territory| &territory.name[..])
                .collect::<Vec<&str>>()
                .join(", "),
            self.continents
                .borrow()
                .iter()
                .map(|continent| &continent.name[..])
                .collect::<Vec<&str>>()
                .join(", "),
        )
    }
}

/// Generated ids for a list of all players. We use the player index as ID
pub fn generate_ids(players: &Vec<Rc<PlayerStruct>>) {
    for (i, player) in enumerate(players) {
        *player.index.borrow_mut() = i;
    }
}
