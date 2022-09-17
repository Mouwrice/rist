use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use itertools::{enumerate, join};

/// Struct representing a continent.
/// A continent groups territories together.
/// `Continent` gets used with an `Rc` and can therefore only have mutable fields with a `RefCell`
#[derive(Debug)]
pub struct Continent {
    pub index: RefCell<usize>,
    pub name: String,
    /// Keeps track of occupied territories per player, using the player id
    pub territories_per_player: RefCell<Vec<u32>>,
    /// Armies rewarded for occupying the entire continent
    _armies_reward: u32,
    /// The amount of territories the continent contains
    pub size: u32,
}

impl Continent {
    /// Creates an instance of a continent
    pub fn new(name: &str, players: usize, armies_reward: u32, size: u32) -> Continent {
        Continent {
            index: RefCell::from(0),
            name: String::from(name),
            territories_per_player: RefCell::from(vec![0; players]),
            _armies_reward: armies_reward,
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
            self.index.borrow(),
            join(&*self.territories_per_player.borrow(), ", "),
            self.size
        )
    }
}

impl Hash for Continent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.borrow().hash(state);
    }
}

impl PartialEq<Self> for Continent {
    fn eq(&self, other: &Self) -> bool {
        *self.index.borrow() == *other.index.borrow()
    }
}

impl Eq for Continent {}

/// Generate ids for a list of all continents.
/// We use the continent index number as ID
pub fn generate_ids(continents: &Vec<&Rc<Continent>>) {
    for (i, continent) in enumerate(continents) {
        *continent.index.borrow_mut() = i;
    }
}
