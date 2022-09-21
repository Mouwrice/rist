use crate::boards::BoardStruct;
use crate::dice::{player_rolls_dice, players_roll_die};
use crate::players::PlayerStruct;
use itertools::enumerate;
use std::cell::RefCell;
use std::cmp::{max, min};

use crate::territory::Territory;
use std::rc::Rc;
use std::time::{Duration, SystemTime};

pub mod boards;
pub mod continent;
pub mod dice;
pub mod players;
pub mod territory;

pub struct Attack {
    dice: u32,
    attacker: Rc<Territory>,
    defender: Rc<Territory>,
}

pub struct Game {
    players: Vec<Rc<PlayerStruct>>,
    board: BoardStruct,
    defeated_players: usize,
    armies_in_box: RefCell<Vec<u32>>,
}

impl Game {
    pub fn new(players: Vec<Rc<PlayerStruct>>, board: BoardStruct) -> Game {
        let armies_in_box: Vec<u32> = players.iter().map(|_| 180).collect();
        players::generate_ids(&players);
        Game {
            players,
            board,
            defeated_players: 0,
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
        for player in &*self.players {
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
            self.board
                .claim_territory(free_territory_index, Rc::clone(player));

            // Get the next player
            player = &self.players[(&*player.index.borrow() + 1) % self.players.len()];
        }
    }

    /// Starts the actual game and game loop
    pub fn play(&mut self) {
        let mut player = &self.players[first_player(&self.players.iter().collect())];

        let mut turn = 1;

        let start = SystemTime::now();
        let mut duration;

        loop {
            duration = SystemTime::now().duration_since(start).unwrap();
            if SystemTime::now().duration_since(start).unwrap() > Duration::from_secs(10) {
                break;
            }

            if !*player.defeated.borrow() {
                self.board
                    .set_extra_info(format!("TURN {turn}: {}", player.name));
                println!("{}", self.board);
                self.board.clear_extra_info();

                self.army_accumulation(player);

                self.army_placement(player);

                for defeated in self.attack(Rc::clone(player)).iter() {
                    *defeated.defeated.borrow_mut() = true;
                    self.defeated_players += 1;
                }

                if self.defeated_players == (&self.players.len() - 1) {
                    for territory in &self.board.territories {
                        assert!(
                            territory.get_player().is_some(),
                            "Oops. Not all territories are occupied by the winner..."
                        );
                        assert_eq!(
                            territory.get_player().unwrap(),
                            *player,
                            "Oops. Not all territories are occupied by the winner..."
                        )
                    }
                    self.board
                        .set_extra_info(format!("{} HAS WON THE GAME!", player.name));
                    break;
                }

                // free move
                // TODO

                turn += 1;
            }

            // Get the next player
            player = &self.players[(&*player.index.borrow() + 1) % self.players.len()];
        }

        println!("Game took {} seconds.", duration.as_secs());
        println!("Played {turn} turns in {} seconds.", duration.as_secs());
        println!("Average turn took {} seconds.", duration.as_secs() / turn);
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

        // Per continent rewards
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

    fn army_placement(&self, player: &Rc<PlayerStruct>) {
        self.board.clear_extra_info();
        self.board.set_extra_info(String::from("Army Placement:"));

        let placement = player.place_armies(&self.board);

        if placement.is_empty() {
            self.board.set_extra_info(String::from("No armies placed."));
            println!("{}", self.board);
            self.board.clear_extra_info();
            return;
        }

        self.board
            .set_extra_info(format!("{} places armies on:", player.name));

        for (territory, armies) in placement.iter() {
            territory.place_armies(Rc::clone(player), *armies);
            self.board
                .set_extra_info(format!("\t{}: {armies}", territory.name));
        }

        println!("{}", self.board);
        self.board.clear_extra_info();
    }

    /// Attacking phase
    /// Returns a list of defeated players
    fn attack(&self, player: Rc<PlayerStruct>) -> Vec<Rc<PlayerStruct>> {
        let mut defeated = vec![];

        while let Some(attack) = player.attack(&self.board) {
            assert!(attack.attacker.get_player().is_some());

            // The player who attacks
            let aggressor = attack.attacker.get_player().unwrap();

            assert_eq!(
                aggressor, player,
                "The attacking territory is not claimed by {}",
                player.name
            );

            assert!(0 < attack.dice && attack.dice < *attack.attacker.armies.borrow(), "Number of dies should be between 1 and 3 and one less than the amount of armies on the territory.\narmies: {}\ndie: {}\n", *attack.attacker.armies.borrow(), attack.dice);

            assert!(attack.defender.get_player().is_some());

            let defender = attack.defender.get_player().unwrap();

            assert_ne!(
                aggressor, defender,
                "You cannot attack a territory you own."
            );

            let defense = defender.defend(&attack);
            assert!(
                0 < defense && defense <= max(2, *aggressor.armies.borrow()),
                "Incorrect number of dice used by the defender."
            );

            // Simulate dice rolls
            let mut attacker_rolls = player_rolls_dice(&*aggressor, attack.dice);
            let mut defender_rolls = player_rolls_dice(&*defender, defense);

            attacker_rolls.sort();
            defender_rolls.sort();

            let mut attacker_losses = 0;
            let mut defender_losses = 0;
            for i in 0..min(attacker_rolls.len(), defender_rolls.len()) {
                if attacker_rolls[i] > defender_rolls[i] {
                    defender_losses += 1;
                } else {
                    attacker_losses += 1;
                }
            }

            // Remove killed armies
            *attack.attacker.armies.borrow_mut() -= attacker_losses;
            *attack.defender.armies.borrow_mut() -= defender_losses;

            // Add killed armies back into the box
            self.armies_in_box.borrow_mut()[*aggressor.index.borrow()] += attacker_losses;
            self.armies_in_box.borrow_mut()[*defender.index.borrow()] += defender_losses;

            // The defender loses the territory
            if *attack.defender.armies.borrow() == 0 {
                self.board.set_extra_info(format!(
                    "{} has defeated all armies and captures {}",
                    attack.attacker.name, attack.defender.name
                ));

                // The defender loses a continent
                if defender
                    .continents
                    .borrow()
                    .contains(&*attack.defender.continent)
                {
                    defender
                        .continents
                        .borrow_mut()
                        .remove(&(attack.defender.continent));
                }

                defender.territories.borrow_mut().remove(&*attack.defender);
                aggressor
                    .territories
                    .borrow_mut()
                    .insert(Rc::clone(&attack.defender));

                self.board.set_extra_info(String::from("Losses:"));
                if attacker_losses > 0 {
                    self.board.set_extra_info(format!(
                        "\tAttacker lost {attacker_losses} armies. {} remaining on {}",
                        *attack.attacker.armies.borrow(),
                        attack.attacker.name
                    ));
                }

                self.board.print_board(Duration::from_secs(1));
                self.board.clear_extra_info();

                let capture = aggressor.capture(&attack);
                assert!(capture > attack.dice, "You must move into the territory with at least as many armies as the number of dice rolled.");
                assert!(capture < *attack.attacker.armies.borrow(), "Not enough armies on territory. No territory may ever be left unoccupied at any time during the game.");

                // Move armies from the attacking territory to the captured territory
                *attack.defender.armies.borrow_mut() = capture;
                *attack.attacker.armies.borrow_mut() -= capture;

                attack
                    .defender
                    .continent
                    .territories_per_player
                    .borrow_mut()[*defender.index.borrow()] -= 1;
                attack
                    .attacker
                    .continent
                    .territories_per_player
                    .borrow_mut()[*aggressor.index.borrow()] += 1;

                // The attacker captures an entire continent
                if attack.defender.continent.territories_per_player.borrow()
                    [*aggressor.index.borrow()]
                    == attack.defender.continent.size
                {
                    self.board.set_extra_info(format!(
                        "{} has taken over the entirety of {}",
                        aggressor.name, attack.defender.continent.name
                    ));
                    aggressor
                        .continents
                        .borrow_mut()
                        .insert(Rc::clone(&attack.defender.continent));

                    self.board.print_board(Duration::from_secs(1));
                    self.board.clear_extra_info();
                }

                // The defender has no more territories and is thus defeated
                if defender.territories.borrow().is_empty() {
                    self.board.set_extra_info(format!(
                        "{} IS DEFEATED BY {}!",
                        defender.name, aggressor.name
                    ));
                    self.board.print_board(Duration::from_secs(2));
                    self.board.clear_extra_info();
                    // Remove the defeated player from the list of players
                    defeated.push(defender);
                }
            } else {
                self.board.set_extra_info(format!(
                    "{} was not able to take {}",
                    aggressor.name, attack.defender.name
                ));
                self.board.set_extra_info(format!(
                    "Defender lost {defender_losses} armies. {} remaining on {}",
                    *attack.defender.armies.borrow(),
                    attack.defender.name
                ));
            }
        }
        defeated
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
