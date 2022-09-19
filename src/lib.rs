use crate::boards::BoardStruct;
use crate::dice::players_roll_die;
use crate::players::PlayerStruct;
use itertools::enumerate;
use std::rc::Rc;

pub mod boards;
pub mod continent;
pub mod dice;
pub mod players;
pub mod territory;

pub struct Game {
    players: Vec<Rc<PlayerStruct>>,
    defeated_players: Vec<PlayerStruct>,
    board: BoardStruct,
    game_over: bool,
    armies_in_box: Vec<u32>,
}

impl Game {
    pub fn new(players: Vec<Rc<PlayerStruct>>, board: BoardStruct) -> Game {
        let armies_in_box = players.iter().map(|_| 180).collect();
        players::generate_ids(&players);
        Game {
            players,
            defeated_players: vec![],
            board,
            game_over: false,
            armies_in_box,
        }
    }

    pub fn setup(&mut self) {
        println!("--- SETUP ---\n");

        // The total amount of armies a player is entitled to depends on the amount of players.
        // Playing with more than 6 players is not allowed
        let armies_per_player = vec![50, 35, 30, 25, 20][&self.players.len() - 2];
        println!("Armies per player: {}\n", armies_per_player);

        // Every player receives initial amount of armies
        for player in &self.players {
            *player.armies.borrow_mut() = armies_per_player;
            assert!(
                self.armies_in_box[*player.index.borrow()] >= armies_per_player,
                "Not enough armies in the box"
            );
            self.armies_in_box[*player.index.borrow()] -= armies_per_player;
        }

        // Decide who gets to go first
        println!("Highest roller gets to place it's armies first!\n");
        let mut player = &self.players[first_player(&self.players.iter().collect())];
        println!("{} may begin!\n", player.name);

        println!("{}", self.board);

        while !self.board.free_territories.is_empty() {
            assert!(
                self.armies_in_box[*player.index.borrow()] > 0,
                "Not enough armies in the box."
            );

            let free_territory_index = player.claim_territory(&self.board);
            self.board.claim_territory(free_territory_index, player);

            // Get the next player
            player = &self.players[(&*player.index.borrow() + 1) % self.players.len()];
        }
    }
}

/// Decides which player gets to go first based on random dice rolls
pub fn first_player(players: &Vec<&Rc<PlayerStruct>>) -> usize {
    let mut rolls = players_roll_die(players);
    let mut players_index: Vec<usize> = vec![];
    for (index, _) in enumerate(players) {
        players_index.push(index);
    }

    let mut highest = rolls[0];
    let mut i = 1;
    let mut l = 0;
    while players_index.len() > 1 {
        if rolls[i] < highest {
            players_index.remove(i);
            rolls.remove(i);
        } else {
            highest = rolls[i];
            i += 1;
        }
        if 1 < players_index.len() && players_index.len() == i {
            i = 0;
            l += 1;
            if l == 2 {
                l = 0;
                println!("\nThere is a tie! Re-rolling...\n");
                let new_players = players_index.iter().map(|index| players[*index]).collect();
                rolls = players_roll_die(&new_players)
            }
        }
    }

    players_index[0]
}
