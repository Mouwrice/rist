use rist::continent::{generate_ids, Continent};

/// Test the continent display visually
#[test]
fn test_continent_display() {
    let continent = Continent::new("TestContinent", 4, 6);
    println!("{:#?}", continent);
    println!("{}", continent);
}

/// Test the continent numbering
#[test]
fn test_continent_id_generation() {
    let continent1 = Continent::new("TestContinent1", 4, 4);
    let continent2 = Continent::new("TestContinent2", 4, 5);
    let continent3 = Continent::new("TestContinent3", 4, 6);

    generate_ids(vec![&continent1, &continent2, &continent3]);

    assert_eq!(*continent1.id.borrow(), 0);
    assert_eq!(*continent2.id.borrow(), 1);
    assert_eq!(*continent3.id.borrow(), 2);
}
