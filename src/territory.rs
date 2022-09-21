use std::cell::RefCell;
use std::cmp::min;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

use itertools::enumerate;

use crate::continent::Continent;
use crate::players::PlayerStruct;

#[derive(Debug)]
/// Represents a singular Risk Territory.
/// `Territory` gets used with an `Rc` and can therefore only have mutable fields with a `RefCell`
pub struct Territory {
    pub index: RefCell<usize>,
    pub name: String,
    pub abbr: String,
    pub connections: RefCell<Vec<Weak<Territory>>>,
    pub continent: Rc<Continent>,
    pub armies: RefCell<u32>,
    pub player: RefCell<Option<Weak<PlayerStruct>>>,
}

impl Territory {
    pub fn new(name: &str, continent: Rc<Continent>) -> Self {
        Territory {
            index: RefCell::from(0),
            name: String::from(name),
            abbr: String::from(&name[0..min(5, name.len())]).to_uppercase(),
            connections: RefCell::from(vec![]),
            continent,
            armies: RefCell::from(0),
            player: RefCell::from(None),
        }
    }

    /// Creates the connections to the given territories
    pub fn create_connections(&self, connections: Vec<&Rc<Territory>>) {
        *self.connections.borrow_mut() = connections
            .iter()
            .map(|territory| Rc::downgrade(territory))
            .collect();
    }

    pub fn get_player(&self) -> Option<Rc<PlayerStruct>> {
        if let Some(weak) = &*self.player.borrow() {
            let player = weak.upgrade();
            return player;
        }
        None
    }

    /// Places given amount from armies on the territory and removes them from the player
    /// Territory must be owned by the player or not owned at all
    pub fn place_armies(&self, player: Rc<PlayerStruct>, armies: u32) {
        if let Some(occupant) = self.get_player() {
            assert_eq!(occupant, player, "Territory is owned by another player");
        }
        assert!(
            *player.armies.borrow() > armies,
            "The player does not have enough armies available"
        );

        *self.armies.borrow_mut() += armies;
        *player.armies.borrow_mut() -= armies;
    }
}

impl Display for Territory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut connections = vec![];
        for territory in &*self.connections.borrow() {
            if let Some(territory) = territory.upgrade() {
                connections.push(String::from(&territory.name));
            }
        }
        let connections = connections.join(", ");

        let mut player_name = String::from("None");
        if let Some(player) = self.get_player() {
            player_name = String::from(&player.name);
        }

        write!(
            f,
            "{}\n\
            \tid: {}\n\
            \tcontinent: {}\n\
            \tconnections: {}\n\
            \tarmies: {}\n\
            \tplayer: {}\n",
            self.name,
            self.index.borrow(),
            self.continent.name,
            connections,
            self.armies.borrow(),
            player_name
        )
    }
}

impl Hash for Territory {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.borrow().hash(state);
    }
}

impl PartialEq<Self> for Territory {
    fn eq(&self, other: &Self) -> bool {
        *self.index.borrow() == *other.index.borrow()
    }
}

impl Eq for Territory {}

/// Generate ids for a list of all territories.
/// We use the territory index number as ID, which is used later for fast territory lookup
pub fn generate_ids(territories: &Vec<&Rc<Territory>>) {
    for (i, territory) in enumerate(territories) {
        *territory.index.borrow_mut() = i;
    }
}
