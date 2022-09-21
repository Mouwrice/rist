use rist::boards::BoardStruct;
use rist::boards::BoardType::Unimplemented;
use std::rc::Rc;

use rist::continent::Continent;
use rist::territory::Territory;

/// Test `BoardStruct` creation
#[test]
fn test_new_board_struct() {
    let continent1 = Rc::new(Continent::new("TestContinent1", 4, 5, 3));
    let continent2 = Rc::new(Continent::new("TestContinent2", 4, 3, 2));

    let territory1 = Rc::new(Territory::new("TestTerritory1", Rc::clone(&continent1)));
    let territory2 = Rc::new(Territory::new("TestTerritory2", Rc::clone(&continent1)));
    let territory3 = Rc::new(Territory::new("TestTerritory3", Rc::clone(&continent1)));

    let territory4 = Rc::new(Territory::new("TestTerritory4", Rc::clone(&continent2)));
    let territory5 = Rc::new(Territory::new("TestTerritory5", Rc::clone(&continent2)));

    territory1.create_connections(vec![&territory2, &territory3]);
    territory2.create_connections(vec![&territory1, &territory4]);
    territory3.create_connections(vec![&territory1, &territory5]);

    territory4.create_connections(vec![&territory2, &territory5]);
    territory5.create_connections(vec![&territory4, &territory3]);

    let board = BoardStruct::generate_board(
        Unimplemented,
        vec![&continent1, &continent2],
        vec![
            &territory1,
            &territory2,
            &territory3,
            &territory4,
            &territory5,
        ],
        0,
    );

    println!("{:#?}", board);
}

/// Tests claiming territories
mod claim_territory {
    use colored::Color::{Magenta, White};
    use rist::boards::{BoardStruct, BoardType};
    use rist::continent::Continent;
    use rist::players::{PlayerStruct, PlayerType};
    use rist::territory::Territory;
    use std::rc::Rc;

    #[test]
    fn test_claiming_free_territories() {
        let continent = Rc::new(Continent::new("TestContinent", 4, 5, 1));
        let territory = Rc::new(Territory::new("TestTerritory", Rc::clone(&continent)));

        let mut board = BoardStruct::generate_board(
            BoardType::Unimplemented,
            vec![&continent],
            vec![&territory],
            0,
        );

        let player = Rc::new(PlayerStruct::new(
            PlayerType::Unimplemented,
            "TestPlayer",
            Magenta,
            White,
        ));

        board.claim_territory(0, Rc::clone(&player));

        assert_eq!(*territory.armies.borrow(), 1);
        assert_eq!(*player.armies.borrow(), 0);

        let mut name = String::from("");
        if let Some(player) = territory.get_player() {
            name = String::from(&player.name);
        }

        assert_eq!(name, "TestPlayer");
        assert_eq!(territory.continent.territories_per_player.borrow()[0], 1);
        assert!(player.territories.borrow().contains(&territory));
        assert!(player.continents.borrow().contains(&continent));
    }

    #[test]
    #[should_panic(expected = "The player should have at least 1 army in it's inventory.")]
    fn not_enough_armies() {
        let continent = Rc::new(Continent::new("TestContinent", 4, 5, 1));
        let territory = Rc::new(Territory::new("TestTerritory", Rc::clone(&continent)));

        let mut board = BoardStruct::generate_board(
            BoardType::Unimplemented,
            vec![&continent],
            vec![&territory],
            0,
        );

        let player = Rc::new(PlayerStruct::new(
            PlayerType::Unimplemented,
            "TestPlayer",
            Magenta,
            White,
        ));

        board.claim_territory(0, player);
    }
}
