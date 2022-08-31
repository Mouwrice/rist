use std::collections::HashSet;

use crate::continent::Continent;
use crate::territory::Territory;

#[derive(Debug)]
pub struct PlayerStruct<'a> {
    pub id: u32,
    pub name: String,
    pub armies: u32,
    pub territories: HashSet<Territory<'a>>,
    pub continents: HashSet<Continent>,
}

trait Player {
    fn claim_territory() {} // TODO

    fn place_armies() {} // TODO

    fn attack() {} // TODO

    fn capture() {} // TODO

    fn defend() {} // TODO

    fn free_move() {} // TODO
}
