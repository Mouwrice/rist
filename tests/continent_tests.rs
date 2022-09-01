use rist::continent::{generate_ids, Continent};
use std::rc::Rc;

/// Test the continent display visually
#[test]
fn test_continent_display() {
    let continent = Continent::new("TestContinent", 4, 0, 6);
    println!("{:#?}", continent);
    println!("{}", continent);
}

/// Test the continent numbering
#[test]
fn test_continent_id_generation() {
    let continent1 = Rc::new(Continent::new("TestContinent1", 4, 0, 4));
    let continent2 = Rc::new(Continent::new("TestContinent2", 4, 0, 5));
    let continent3 = Rc::new(Continent::new("TestContinent3", 4, 0, 6));

    generate_ids(&vec![&continent1, &continent2, &continent3]);

    assert_eq!(*continent1.index.borrow(), 0);
    assert_eq!(*continent2.index.borrow(), 1);
    assert_eq!(*continent3.index.borrow(), 2);
}
