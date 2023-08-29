use crate::continent::Continent;
use crate::players::PlayerType;
use crate::territory::Territory;
use colored::{Color, ColoredString, Colorize};
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

/// The default internal state of a player
/// `Player` gets used with an `Rc` and can therefore only have mutable fields with a `RefCell`
#[derive(Debug, PartialEq, Eq)]
pub struct PlayerState {
    pub player_type: PlayerType,
    pub index: RefCell<usize>,
    pub name: String,
    pub armies: RefCell<u32>,
    pub(crate) territories: RefCell<Vec<Rc<Territory>>>,
    continents: RefCell<Vec<Rc<Continent>>>,
    foreground: Color,
    background: Color,
    pub defeated: RefCell<bool>,
}

impl PlayerState {
    pub fn new(player: PlayerType, name: &str, background: Color, foreground: Color) -> Self {
        PlayerState {
            player_type: player,
            index: RefCell::from(0),
            name: String::from(name),
            armies: RefCell::from(0),
            territories: RefCell::from(vec![]),
            continents: RefCell::from(vec![]),
            foreground,
            background,
            defeated: RefCell::from(false),
        }
    }

    pub fn get_territories(&self) -> &RefCell<Vec<Rc<Territory>>> {
        &self.territories
    }

    pub fn add_territory(&self, territory: Rc<Territory>) {
        self.territories.borrow_mut().push(territory);
    }

    pub fn remove_territory(&self, item: &Rc<Territory>) {
        self.territories
            .borrow_mut()
            .retain(|territory| *territory != *item);
    }

    pub fn get_continents(&self) -> &RefCell<Vec<Rc<Continent>>> {
        &self.continents
    }

    pub fn add_continent(&self, continent: Rc<Continent>) {
        self.continents.borrow_mut().push(continent);
    }

    pub fn remove_continent(&self, item: &Rc<Continent>) {
        self.continents
            .borrow_mut()
            .retain(|continent| *continent != *item);
    }

    /// Color the text to the color of the player
    pub fn colorize(&self, text: String) -> ColoredString {
        text.color(self.foreground).on_color(self.background)
    }
}

impl Display for PlayerState {
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
            self.player_type,
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
