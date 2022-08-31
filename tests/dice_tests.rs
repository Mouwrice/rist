use rist::dice;
use rist::player::PlayerStruct;

/// Every dice roll should be between 1 and 6
#[test]
fn test_roll_dice() {
    for roll in dice::roll_dice(10) {
        assert!((1..=6).contains(&roll));
    }
}

/// Visual check to see if the printed text is correct
#[test]
fn test_player_rolls_dice() {
    let player = PlayerStruct {
        id: 0,
        name: "TestPlayer".to_string(),
        armies: 0,
        territories: Default::default(),
        continents: Default::default(),
    };
    assert!((1..=6).contains(&dice::player_rolls_dice(&player, 1)[0]));
    for roll in dice::player_rolls_dice(&player, 10) {
        assert!((1..=6).contains(&roll));
    }
}

/// Visual check to see if the printed text is correct
#[test]
fn test_players_roll_die() {
    let player1 = PlayerStruct {
        id: 0,
        name: "TestPlayer1".to_string(),
        armies: 0,
        territories: Default::default(),
        continents: Default::default(),
    };
    let player2 = PlayerStruct {
        id: 0,
        name: "TestPlayer2".to_string(),
        armies: 0,
        territories: Default::default(),
        continents: Default::default(),
    };
    for roll in dice::players_roll_die(&vec![&player1, &player2]) {
        assert!((1..=6).contains(&roll));
    }
}
