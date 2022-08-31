use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::{Rc, Weak};

use itertools::enumerate;

use crate::continent::Continent;
use crate::player::PlayerStruct;

#[derive(Debug)]
/// Represents a singular Risk Territory
/// `Territory` gets used with an `Rc` and can therefore only have mutable fields with a `RefCell`
pub struct Territory<'a> {
    pub id: RefCell<u32>,
    pub name: String,
    pub abbr: &'a str,
    pub connections: RefCell<Vec<Weak<Territory<'a>>>>,
    pub continent: Rc<Continent>,
    pub armies: u32,
    pub player: Option<&'a PlayerStruct<'a>>,
}

impl<'a> Territory<'a> {
    pub fn new(name: &'a str, continent: Rc<Continent>) -> Territory<'a> {
        Territory {
            id: RefCell::from(0),
            name: String::from(name),
            abbr: &name[0..5],
            connections: RefCell::from(vec![]),
            continent,
            armies: 0,
            player: None,
        }
    }

    /// Creates the connections to the given territories
    pub fn create_connections(&self, connections: Vec<&Rc<Territory<'a>>>) {
        *self.connections.borrow_mut() = connections
            .iter()
            .map(|territory| Rc::downgrade(territory))
            .collect();
    }
}

impl<'a> Display for Territory<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut connections = vec![];
        for territory in &*self.connections.borrow() {
            if let Some(territory) = territory.upgrade() {
                connections.push(String::from(&territory.name));
            }
        }
        let connections = connections.join(", ");
        write!(
            f,
            "{}\n\
            \tid: {}\n\
            \tcontinent: {}\n\
            \tconnections: {}\n\
            \tarmies: {}\n\
            \tplayer: {}\n",
            self.name,
            self.id.borrow(),
            self.continent.name,
            connections,
            self.armies,
            self.player.map(|player| &player.name[..]).unwrap_or("None")
        )
    }
}

/// Generate ids for a list of all territories.
/// We use the territory index number as ID, which is used later for fast territory lookup
pub fn generate_ids(territories: Vec<&Territory>) {
    for (i, territory) in enumerate(territories) {
        *territory.id.borrow_mut() = i as u32;
    }
}
