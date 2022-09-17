use crate::boards::Board;
use crate::players::PlayerStruct;
use std::fmt::Formatter;

pub fn fmt(player: &PlayerStruct, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
        f,
        "{}\n\
            \tindex: {}\n\
            \tarmies: {}\n\
            \tterritories: {}\n\
            \tcontinents: {}",
        player.name,
        player.index,
        player.armies.borrow(),
        player
            .territories
            .borrow()
            .iter()
            .map(|territory| &territory.name[..])
            .collect::<Vec<&str>>()
            .join(", "),
        player
            .continents
            .borrow()
            .iter()
            .map(|continent| &continent.name[..])
            .collect::<Vec<&str>>()
            .join(", "),
    )
}

pub fn claim_territory(player: &PlayerStruct, board: &dyn Board) {}
