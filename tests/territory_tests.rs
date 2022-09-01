use std::rc::Rc;

use rist::continent::Continent;
use rist::territory::{generate_ids, Territory};

/// Test the territory display visually
#[test]
fn test_territory_display() {
    let continent = Rc::new(Continent::new("TestContinent", 4, 5, 3));

    let territory1 = Rc::new(Territory::new("TestTerritory1", Rc::clone(&continent)));
    let territory2 = Rc::new(Territory::new("TestTerritory2", Rc::clone(&continent)));
    let territory3 = Rc::new(Territory::new("TestTerritory3", Rc::clone(&continent)));

    *territory1.connections.borrow_mut() =
        vec![Rc::downgrade(&territory2), Rc::downgrade(&territory3)];
    *territory2.connections.borrow_mut() = vec![Rc::downgrade(&territory1)];
    *territory3.connections.borrow_mut() = vec![Rc::downgrade(&territory1)];

    println!("{:#?}", territory1);
    println!("{:#?}", territory2);
    println!("{:#?}", territory3);

    println!("{}", territory1);
    println!("{}", territory2);
    println!("{}", territory3);
}

/// Test the connection creation
#[test]
fn test_create_connections() {
    let continent = Rc::new(Continent::new("TestContinent", 4, 5, 3));

    let territory1 = Rc::new(Territory::new("TestTerritory1", Rc::clone(&continent)));
    let territory2 = Rc::new(Territory::new("TestTerritory2", Rc::clone(&continent)));
    let territory3 = Rc::new(Territory::new("TestTerritory3", Rc::clone(&continent)));

    territory1.create_connections(vec![&territory2, &territory3]);
    territory2.create_connections(vec![&territory1]);
    territory3.create_connections(vec![&territory1]);

    println!("{}", territory1);

    if let Some(territory) = territory1.connections.borrow()[0].upgrade() {
        assert_eq!(territory.name, "TestTerritory2");
    };

    if let Some(territory) = territory1.connections.borrow()[1].upgrade() {
        assert_eq!(territory.name, "TestTerritory3");
    };

    if let Some(territory) = territory2.connections.borrow()[0].upgrade() {
        assert_eq!(territory.name, "TestTerritory1");
    };

    if let Some(territory) = territory3.connections.borrow()[0].upgrade() {
        assert_eq!(territory.name, "TestTerritory1");
    };
}

/// Test the territory numbering
#[test]
fn test_territory_id_generation() {
    let continent = Rc::new(Continent::new("TestContinent", 4, 5, 3));

    let territory1 = Rc::new(Territory::new("TestTerritory1", Rc::clone(&continent)));
    let territory2 = Rc::new(Territory::new("TestTerritory2", Rc::clone(&continent)));
    let territory3 = Rc::new(Territory::new("TestTerritory3", Rc::clone(&continent)));

    generate_ids(&vec![&territory1, &territory2, &territory3]);

    assert_eq!(*territory1.index.borrow(), 0);
    assert_eq!(*territory2.index.borrow(), 1);
    assert_eq!(*territory3.index.borrow(), 2);
}
