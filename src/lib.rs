use indicatif::{ProgressBar, ProgressStyle};
use std::cell::RefCell;
use std::cmp::{max, min};
use std::rc::Rc;
use std::time::{Duration, SystemTime};

use itertools::enumerate;

use crate::boards::BoardStruct;
use crate::dice::{player_rolls_dice, players_roll_die};
use crate::players::Player;
use crate::territory::Territory;

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
    players: Vec<Rc<dyn Player>>,
    board: BoardStruct,
    defeated_players: usize,
    armies_in_box: RefCell<Vec<u32>>,
}

impl Game {
    pub fn new(players: Vec<Rc<dyn Player>>, board: BoardStruct) -> Game {
        let armies_in_box: Vec<u32> = players.iter().map(|_| 180).collect();
        players::generate_ids(&players);
        Game {
            players,
            board,
            defeated_players: 0,
            armies_in_box: RefCell::from(armies_in_box),
        }
    }

    pub fn assert_enough_armies(&self, index: usize, armies: u32) {
        assert!(
            self.armies_in_box.borrow()[index] > 0,
            "Not enough armies in the box. Tried to take {} armies when only {} were available.",
            armies,
            self.armies_in_box.borrow()[index]
        );
    }

    /// The game setup hands out the initial amount of armies to the players
    /// and lets the players claim their first territories
    /// `verbose` whether the function should output to stdout
    pub fn setup(&mut self, verbose: bool) {
        if verbose {
            println!("--- SETUP ---\n");
        }

        // The total amount of armies a player is entitled to depends on the amount of players.
        // Playing with more than 6 players is not allowed
        let armies_per_player = vec![50, 35, 30, 25, 20][&self.players.len() - 2];
        if verbose {
            println!("Armies per player: {}\n", armies_per_player);
        }

        // Every player receives initial amount of armies
        for player in &*self.players {
            let state = player.get_state();
            *state.armies.borrow_mut() = armies_per_player;
            assert!(
                self.armies_in_box.borrow()[*state.index.borrow()] >= armies_per_player,
                "Not enough armies in the box"
            );
            self.armies_in_box.borrow_mut()[*state.index.borrow()] -= armies_per_player;
        }

        // Decide who gets to go first
        if verbose {
            println!("Highest roller gets to place it's armies first!\n");
        }
        let mut player = &self.players[first_player(&self.players.iter().collect(), verbose)];
        if verbose {
            println!("{} may begin!\n", player.get_state().name);
            self.board.print_board();
        }

        while !self.board.free_territories.is_empty() {
            assert!(
                self.armies_in_box.borrow()[*player.get_state().index.borrow()] > 0,
                "Not enough armies in the box., Tried to take {} armies when only {} were available.",
                1,
                self.armies_in_box.borrow()[*player.get_state().index.borrow()]
            );

            let free_territory_index = player.claim_territory(&self.board);
            self.board
                .claim_territory(free_territory_index, Rc::clone(player), verbose);

            // Get the next player
            player = &self.players[(&*player.get_state().index.borrow() + 1) % self.players.len()];
        }

        for continent in &self.board.continents {
            assert_eq!(
                continent
                    .territories_per_player
                    .borrow()
                    .iter()
                    .sum::<u32>(),
                continent.size,
                "{}",
                continent
            );
        }
    }

    /// Starts the actual game and game loop
    pub fn play(
        &mut self,
        max_duration: Option<Duration>,
        max_turns: Option<u64>,
        verbose: bool,
        with_progressbar: bool,
    ) {
        let mut player = &self.players[first_player(&self.players.iter().collect(), verbose)];

        let mut turn = 1;

        let start = SystemTime::now();
        let mut duration;

        let mut progressbar = None;
        if let Some(turns) = max_turns {
            if with_progressbar && !verbose {
                let bar = ProgressBar::new(turns);
                bar.set_style(
                    ProgressStyle::with_template(
                        "{percent}% {wide_bar} {pos}/{len} [{elapsed}<{eta}, {per_sec}] ",
                    )
                    .unwrap(),
                );
                progressbar = Some(bar);
            }
        }

        loop {
            duration = SystemTime::now().duration_since(start).unwrap();
            if let Some(duration) = max_duration {
                if SystemTime::now().duration_since(start).unwrap() > duration {
                    break;
                }
            }

            if let Some(turns) = max_turns {
                if turn >= turns {
                    break;
                }
            }

            if !*player.get_state().defeated.borrow() {
                if verbose {
                    self.board
                        .set_extra_info(format!("TURN {turn}: {}", player.get_state().name));
                    self.board.print_board();
                    self.board.clear_extra_info();
                }

                self.army_accumulation(player, verbose);

                self.army_placement(player, verbose);

                for defeated in self.attack(Rc::clone(player), verbose).iter() {
                    *defeated.get_state().defeated.borrow_mut() = true;
                    self.defeated_players += 1;
                }

                if self.defeated_players == (&self.players.len() - 1) {
                    for territory in &self.board.territories {
                        assert!(
                            territory.get_player().is_some(),
                            "Oops. Not all territories are occupied by the winner..."
                        );
                        assert_eq!(
                            territory.get_player().unwrap().get_state(),
                            player.get_state(),
                            "Oops. Not all territories are occupied by the winner..."
                        )
                    }

                    if verbose {
                        self.board.set_extra_info(format!(
                            "{} HAS WON THE GAME!",
                            player.get_state().name
                        ));
                    }

                    break;
                }

                self.free_move(player, verbose);

                if let Some(bar) = &progressbar {
                    bar.inc(1);
                }

                turn += 1;
            }

            // Get the next player
            player = &self.players[(&*player.get_state().index.borrow() + 1) % self.players.len()];
        }

        if let Some(bar) = progressbar {
            bar.finish();
        }

        println!("Game took {} seconds.", duration.as_secs());
        println!("Played {turn} turns in {} seconds.", duration.as_secs());
        println!("Average turn took {} seconds.", duration.as_secs() / turn);
    }

    /// Calculates how much armies a player is received upon starting its turn
    /// A player is entitled to its amount of territories divided by 3 and reaches a minimum of 3 armies if possible
    fn army_accumulation(&self, player: &Rc<dyn Player>, verbose: bool) {
        if verbose {
            self.board
                .set_extra_info(String::from("Army Accumulation:"));
            self.board.set_extra_info(String::from(""));
        }

        // Out of armies
        if self.armies_in_box.borrow()[*player.get_state().index.borrow()] == 0 {
            if verbose {
                self.board
                    .set_extra_info(String::from("No more armies available in the box."));
                self.board.print_board();
                self.board.clear_extra_info();
            }

            return;
        }

        let mut armies = min(
            self.armies_in_box.borrow()[*player.get_state().index.borrow()],
            max(
                3,
                player.get_state().get_territories().borrow().len() as u32 / 3,
            ),
        );

        if verbose {
            self.board.set_extra_info(format!(
                "{} receives {armies} for occupying {} territories.",
                player.get_state().name,
                player.get_state().get_territories().borrow().len()
            ));
        }

        // Per continent rewards
        for continent in player.get_state().get_continents().borrow().iter() {
            let extra = min(
                self.armies_in_box.borrow()[*player.get_state().index.borrow()] - armies,
                continent.armies_reward,
            );
            armies += extra;

            if verbose {
                self.board.set_extra_info(format!(
                    "{} receives {extra} armies for occupying the entirety of {}.",
                    player.get_state().name,
                    continent.name
                ));
            }
        }

        // Assign armies
        *player.get_state().armies.borrow_mut() += armies;

        // Remove assigned armies from the box
        self.assert_enough_armies(*player.get_state().index.borrow(), armies);
        self.armies_in_box.borrow_mut()[*player.get_state().index.borrow()] -= armies;

        if verbose {
            self.board.set_extra_info(format!(
                "{} has received a total of {armies} armies.",
                player.get_state().name,
            ));
            self.board.print_board();
            self.board.clear_extra_info();
        }
    }

    fn army_placement(&self, player: &Rc<dyn Player>, verbose: bool) {
        if verbose {
            self.board.clear_extra_info();
            self.board.set_extra_info(String::from("Army Placement:"));
            self.board.set_extra_info(String::from(""));
        }

        let placement = player.place_armies(&self.board);

        if placement.is_empty() {
            if verbose {
                self.board.set_extra_info(String::from("No armies placed."));
                self.board.print_board();
                self.board.clear_extra_info();
            }
            return;
        }

        if verbose {
            self.board
                .set_extra_info(format!("{} places armies on:", player.get_state().name));
        }

        for (territory, armies) in placement.iter() {
            territory.place_armies(Rc::clone(player), *armies);
            if verbose {
                self.board
                    .set_extra_info(format!(" * {} +{armies}", territory.name));
            }
        }

        if verbose {
            self.board.print_board();
            self.board.clear_extra_info();
        }
    }

    /// Attacking phase
    /// Returns a list of defeated players
    fn attack(&self, player: Rc<dyn Player>, verbose: bool) -> Vec<Rc<dyn Player>> {
        let mut defeated = vec![];

        while let Some(attack) = player.attack(&self.board) {
            assert!(attack.attacker.get_player().is_some());

            // The player who attacks
            let aggressor = attack.attacker.get_player().unwrap();

            assert_eq!(
                aggressor.get_state(),
                player.get_state(),
                "The attacking territory is not claimed by {}",
                player.get_state().name
            );

            assert!(0 < attack.dice && attack.dice < *attack.attacker.armies.borrow(), "Number of dies should be between 1 and 3 and one less than the amount of armies on the territory.\narmies: {}\ndie: {}\n", *attack.attacker.armies.borrow(), attack.dice);

            assert!(attack.defender.get_player().is_some());

            let defender = attack.defender.get_player().unwrap();

            assert_ne!(
                aggressor.get_state(),
                defender.get_state(),
                "You cannot attack a territory you own."
            );

            let defense = defender.defend(&attack);
            assert!(
                0 < defense && defense <= max(2, *aggressor.get_state().armies.borrow()),
                "Incorrect number of dice used by the defender."
            );

            if verbose {
                self.board.set_extra_info(format!(
                    "{} attacks {} with {} armies",
                    attack.attacker.name, attack.defender.name, attack.dice
                ));
                self.board.set_extra_info(String::from(""));
                self.board.set_extra_info(format!(
                    "{} defends with {} armies",
                    attack.defender.name, defense
                ));
                self.board.set_extra_info(String::from(""));
            }

            // Simulate dice rolls
            let mut attacker_rolls = player_rolls_dice(aggressor.as_ref(), attack.dice, false);
            let mut defender_rolls = player_rolls_dice(defender.as_ref(), defense, false);

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
            self.armies_in_box.borrow_mut()[*aggressor.get_state().index.borrow()] +=
                attacker_losses;
            self.armies_in_box.borrow_mut()[*defender.get_state().index.borrow()] +=
                defender_losses;

            if verbose {
                self.board.set_extra_info(format!(
                    "Attacker lost {attacker_losses} armies. {} remaining on {}.",
                    *attack.attacker.armies.borrow(),
                    attack.attacker.name
                ));

                self.board.set_extra_info(format!(
                    "Defender lost {defender_losses} armies. {} remaining on {}.",
                    *attack.defender.armies.borrow(),
                    attack.defender.name
                ));
            }

            // The defender loses the territory
            if *attack.defender.armies.borrow() == 0 {
                if verbose {
                    self.board.set_extra_info(format!(
                        "{} has defeated all armies and captures {}.",
                        attack.attacker.name, attack.defender.name
                    ));
                }

                // The defender loses a continent
                if defender
                    .get_state()
                    .get_continents()
                    .borrow()
                    .contains(&attack.defender.continent)
                {
                    defender
                        .get_state()
                        .remove_continent(&attack.defender.continent);
                }

                defender.get_state().remove_territory(&attack.defender);
                aggressor
                    .get_state()
                    .add_territory(Rc::clone(&attack.defender));

                attack.defender.set_player(Some(Rc::downgrade(&aggressor)));

                let capture = aggressor.capture(&attack);
                assert!(capture >= attack.dice, "You must move into the territory with at least as many armies as the number of dice rolled.");
                assert!(capture < *attack.attacker.armies.borrow(), "Not enough armies on territory. No territory may ever be left unoccupied at any time during the game.");

                if verbose {
                    self.board.print_board();
                    self.board.clear_extra_info();
                }

                // Move armies from the attacking territory to the captured territory
                *attack.defender.armies.borrow_mut() = capture;
                *attack.attacker.armies.borrow_mut() -= capture;

                if attack.defender.continent.territories_per_player.borrow()
                    [*defender.get_state().index.borrow()]
                    == 0
                {
                    println!("{}", attack.defender.continent);
                    println!("{}", defender.get_state());
                }
                attack
                    .defender
                    .continent
                    .territories_per_player
                    .borrow_mut()[*defender.get_state().index.borrow()] -= 1;
                attack
                    .defender
                    .continent
                    .territories_per_player
                    .borrow_mut()[*aggressor.get_state().index.borrow()] += 1;

                // The attacker captures an entire continent
                if attack.defender.continent.territories_per_player.borrow()
                    [*aggressor.get_state().index.borrow()]
                    == attack.defender.continent.size
                {
                    aggressor
                        .get_state()
                        .add_continent(Rc::clone(&attack.defender.continent));

                    if verbose {
                        self.board.set_extra_info(format!(
                            "{} has taken over the entirety of {}",
                            aggressor.get_state().name,
                            attack.defender.continent.name
                        ));
                        self.board.print_board();
                        self.board.clear_extra_info();
                    }
                }

                // The defender has no more territories and is thus defeated
                if defender.get_state().get_territories().borrow().is_empty() {
                    // Remove the defeated player from the list of players
                    defeated.push(Rc::clone(&defender));

                    if verbose {
                        self.board.set_extra_info(format!(
                            "{} IS DEFEATED BY {}!",
                            defender.get_state().name,
                            aggressor.get_state().name
                        ));
                        self.board.print_board();
                        self.board.clear_extra_info();
                    }
                }
            } else if verbose {
                self.board.set_extra_info(format!(
                    "{} was not able to take {}.",
                    aggressor.get_state().name,
                    attack.defender.name
                ));
            }

            if verbose {
                self.board.print_board();
                self.board.clear_extra_info();
            }
        }
        defeated
    }

    fn free_move(&self, _player: &Rc<dyn Player>, _verbose: bool) {
        // TODO: Implement free move
    }
}

/// Decides which player gets to go first based on random dice rolls
pub fn first_player(players: &Vec<&Rc<dyn Player>>, verbose: bool) -> usize {
    let mut rolls = players_roll_die(players, verbose);
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
                if verbose {
                    println!("\nThere is a tie! Re-rolling...\n");
                }
                let new_players = players_index.iter().map(|index| players[*index]).collect();
                rolls = players_roll_die(&new_players, verbose)
            }
        }
    }

    players_index[0]
}
