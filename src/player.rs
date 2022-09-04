use colored::{Color, ColoredString, Colorize};
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::continent::Continent;
use crate::territory::Territory;

#[derive(Debug)]
/// `Player` gets used with an `Rc` and can therefore only have mutable fields with a `RefCell`
pub struct PlayerStruct {
    pub index: usize,
    pub name: String,
    pub armies: RefCell<u32>,
    pub territories: RefCell<HashSet<Rc<Territory>>>,
    pub continents: RefCell<HashSet<Rc<Continent>>>,
    foreground: Color,
    background: Color,
}

impl PlayerStruct {
    pub fn new(name: &str, armies: u32, background: Color, foreground: Color) -> Self {
        PlayerStruct {
            index: 0,
            name: String::from(name),
            armies: RefCell::from(armies),
            territories: RefCell::from(HashSet::new()),
            continents: RefCell::from(HashSet::new()),
            foreground,
            background,
        }
    }
}

/// Color the text to the color of the player
pub fn colorize(player: &Rc<PlayerStruct>, text: String) -> ColoredString {
    text.color(player.foreground).on_color(player.background)
}

impl Display for PlayerStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n\
            \tindex: {}\n\
            \tarmies: {}\n\
            \tterritories: {}\n\
            \tcontinents: {}",
            self.name,
            self.index,
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

trait Player {
    fn claim_territory();

    fn place_armies();

    fn attack();

    fn capture();

    fn defend();

    fn free_move();

    fn colorize(player: &PlayerStruct, text: String);
}
