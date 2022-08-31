use std::cell::RefCell;
use std::fmt::{Display, Formatter};

use itertools::{enumerate, join};

/// Struct representing a continent
/// A continent groups territories together
/// /// `Continent` gets used with an `Rc` and can therefore only have mutable fields with a `RefCell`
#[derive(Debug)]
pub struct Continent {
    pub id: RefCell<u32>,
    pub name: String,
    /// Keeps track of occupied territories per player, using the player id
    territories_per_player: Vec<u32>,
    /// The amount of territories the continent contains
    size: u32,
}

impl Continent {
    /// Creates an instance of a continent
    pub fn new(name: &str, players: usize, size: u32) -> Continent {
        Continent {
            id: RefCell::from(0),
            name: String::from(name),
            territories_per_player: vec![0; players],
            size,
        }
    }
}

impl Display for Continent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n\
            \tid: {}\n\
            \tterritories_per_player: {}\n\
            \tsize: {}",
            self.name,
            self.id.borrow(),
            join(&self.territories_per_player, ", "),
            self.size
        )
    }
}

/// Generate ids for a list of all continents.
/// We use the continent index number as ID
pub fn generate_ids(continents: Vec<&Continent>) {
    for (i, continent) in enumerate(continents) {
        *continent.id.borrow_mut() = i as u32;
    }
}
