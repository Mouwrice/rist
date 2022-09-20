use crate::boards::BoardStruct;
use crate::dice::players_roll_die;
use crate::players::PlayerStruct;
use itertools::enumerate;
use std::cell::RefCell;
use std::cmp::{max, min};
use std::rc::Rc;
use std::time::SystemTime;

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
    armies_in_box: RefCell<Vec<u32>>,
}

impl Game {
    pub fn new(players: Vec<Rc<PlayerStruct>>, board: BoardStruct) -> Game {
        let armies_in_box: Vec<u32> = players.iter().map(|_| 180).collect();
        players::generate_ids(&players);
        Game {
            players,
            defeated_players: vec![],
            board,
            game_over: false,
            armies_in_box: RefCell::from(armies_in_box),
        }
    }

    /// The game setup hands out the initial amount of armies to the players
    /// and lets the players claim their first territories
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
                self.armies_in_box.borrow()[*player.index.borrow()] >= armies_per_player,
                "Not enough armies in the box"
            );
            self.armies_in_box.borrow_mut()[*player.index.borrow()] -= armies_per_player;
        }

        // Decide who gets to go first
        println!("Highest roller gets to place it's armies first!\n");
        let mut player = &self.players[first_player(&self.players.iter().collect())];
        println!("{} may begin!\n", player.name);

        println!("{}", self.board);

        while !self.board.free_territories.is_empty() {
            assert!(
                self.armies_in_box.borrow()[*player.index.borrow()] > 0,
                "Not enough armies in the box."
            );

            let free_territory_index = player.claim_territory(&self.board);
            self.board.claim_territory(free_territory_index, player);

            // Get the next player
            player = &self.players[(&*player.index.borrow() + 1) % self.players.len()];
        }
    }

    /// Starts the actual game and game loop
    pub fn play(&self) {
        let mut player = &self.players[first_player(&self.players.iter().collect())];

        let mut turn = 1;

        let start = SystemTime::now();
        let duration = 0;

        loop {
            self.board
                .set_extra_info(format!("TURN {turn}: {}", player.name));
            println!("{}", self.board);
            self.board.clear_extra_info();

            self.army_accumulation(player);
        }
    }

    /// Calculates how much armies a player is received upon starting its turn
    /// A player is entitled to its amount of territories divided by 3 and reaches a minimum of 3 armies if possible
    fn army_accumulation(&self, player: &Rc<PlayerStruct>) {
        self.board
            .set_extra_info(String::from("Army Accumulation:"));

        // Out of armies
        if self.armies_in_box.borrow()[*player.index.borrow()] == 0 {
            self.board
                .set_extra_info(String::from("\tNo more armies available in the box."));
            println!("{}", self.board);
            return;
        }

        let mut armies = min(
            self.armies_in_box.borrow()[*player.index.borrow()],
            max(3, player.territories.borrow().len() as u32 / 3),
        );

        self.board.set_extra_info(format!(
            "\t{} receives {armies} for occupying {} territories.",
            player.name,
            player.territories.borrow().len()
        ));

        for continent in player.continents.borrow().iter() {
            let extra = min(
                self.armies_in_box.borrow()[*player.index.borrow()],
                continent.armies_reward,
            );
            armies += extra;

            self.board.set_extra_info(format!(
                "{} receives {extra} armies for occupying the entirety of {}",
                player.name, continent.name
            ));
        }

        // Assign armies
        *player.armies.borrow_mut() += armies;
        // Remove assigned armies from the box
        self.armies_in_box.borrow_mut()[*player.index.borrow()] -= armies;

        self.board.set_extra_info(format!(
            "{} has received a total of {armies} armies. {} now has {} armies.",
            player.name,
            player.name,
            *player.armies.borrow()
        ));
        println!("{}", self.board);
        self.board.clear_extra_info();
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
