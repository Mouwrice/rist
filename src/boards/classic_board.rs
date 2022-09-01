//! The original classic Risk board

use std::rc::Rc;

use crate::boards;
use BoardStruct as ClassicBoard;

use crate::boards::{Board, BoardStruct};
use crate::continent::Continent;
use crate::player::PlayerStruct;
use crate::territory::Territory;

impl ClassicBoard {
    fn new(players: usize) -> Self {
        // CONTINENTS
        let north_america = Rc::new(Continent::new("North America", players, 5, 9));
        let europe = Rc::new(Continent::new("Europe", players, 5, 6));
        let asia = Rc::new(Continent::new("Asia", players, 7, 12));
        let south_america = Rc::new(Continent::new("South America", players, 2, 4));
        let africa = Rc::new(Continent::new("Africa", players, 3, 6));
        let australia = Rc::new(Continent::new("Australia", players, 2, 4));

        // TERRITORIES

        // NORTH AMERICA
        let alaska = Rc::new(Territory::new("Alaska", Rc::clone(&north_america)));
        let northwest_territory = Rc::new(Territory::new(
            "Northwest Rc::new(Territory",
            Rc::clone(&north_america),
        ));
        let greenland = Rc::new(Territory::new("Greenland", Rc::clone(&north_america)));
        let alberta = Rc::new(Territory::new("Alberta", Rc::clone(&north_america)));
        let ontario = Rc::new(Territory::new("Ontario", Rc::clone(&north_america)));
        let quebec = Rc::new(Territory::new("Quebec", Rc::clone(&north_america)));
        let western_us = Rc::new(Territory::new("Western US", Rc::clone(&north_america)));
        let eastern_us = Rc::new(Territory::new("Eastern US", Rc::clone(&north_america)));
        let central_america = Rc::new(Territory::new("Central America", Rc::clone(&north_america)));

        // SOUTH AMERICA
        let venezuela = Rc::new(Territory::new("Venezuela", Rc::clone(&south_america)));
        let brazil = Rc::new(Territory::new("Brazil", Rc::clone(&south_america)));
        let peru = Rc::new(Territory::new("Peru", Rc::clone(&south_america)));
        let argentina = Rc::new(Territory::new("Argentina", Rc::clone(&south_america)));

        // EUROPE
        let iceland = Rc::new(Territory::new("Iceland", Rc::clone(&europe)));
        let scandinavia = Rc::new(Territory::new("Scandinavia", Rc::clone(&europe)));
        let great_britain = Rc::new(Territory::new("Great Britain", Rc::clone(&europe)));
        let northern_europe = Rc::new(Territory::new("Northern Europe", Rc::clone(&europe)));
        let ukraine = Rc::new(Territory::new("Ukraine", Rc::clone(&europe)));
        let western_europe = Rc::new(Territory::new("Western Europe", Rc::clone(&europe)));
        let southern_europe = Rc::new(Territory::new("Southern Europe", Rc::clone(&europe)));

        // ASIA
        let yakutsk = Rc::new(Territory::new("Yakutsk", Rc::clone(&asia)));
        let ural = Rc::new(Territory::new("Ural", Rc::clone(&asia)));
        let siberia = Rc::new(Territory::new("Siberia", Rc::clone(&asia)));
        let irkutsk = Rc::new(Territory::new("Irkutsk", Rc::clone(&asia)));
        let kamchatka = Rc::new(Territory::new("Kamchatka", Rc::clone(&asia)));
        let afghanistan = Rc::new(Territory::new("Afghanistan", Rc::clone(&asia)));
        let china = Rc::new(Territory::new("China", Rc::clone(&asia)));
        let mongolia = Rc::new(Territory::new("Mongolia", Rc::clone(&asia)));
        let japan = Rc::new(Territory::new("Japan", Rc::clone(&asia)));
        let middle_east = Rc::new(Territory::new("Middle East", Rc::clone(&asia)));
        let india = Rc::new(Territory::new("India", Rc::clone(&asia)));
        let siam = Rc::new(Territory::new("Siam", Rc::clone(&asia)));

        // AFRICA
        let north_africa = Rc::new(Territory::new("North Africa", Rc::clone(&africa)));
        let egypt = Rc::new(Territory::new("Egypt", Rc::clone(&africa)));
        let congo = Rc::new(Territory::new("Congo", Rc::clone(&africa)));
        let east_africa = Rc::new(Territory::new("East Africa", Rc::clone(&africa)));
        let south_africa = Rc::new(Territory::new("South Africa", Rc::clone(&africa)));
        let madagascar = Rc::new(Territory::new("Madagascar", Rc::clone(&africa)));

        // AUSTRALIA
        let indonesia = Rc::new(Territory::new("Indonesia", Rc::clone(&australia)));
        let new_guinea = Rc::new(Territory::new("New Guinea", Rc::clone(&australia)));
        let western_australia = Rc::new(Territory::new("Western Australia", Rc::clone(&australia)));
        let eastern_australia = Rc::new(Territory::new("Eastern Australia", Rc::clone(&australia)));

        // CONNECTIONS
        alaska.create_connections(vec![&northwest_territory, &alberta, &kamchatka]);
        northwest_territory.create_connections(vec![&alaska, &greenland, &alberta, &ontario]);
        greenland.create_connections(vec![&northwest_territory, &iceland, &ontario, &quebec]);
        alberta.create_connections(vec![&alaska, &northwest_territory, &ontario, &western_us]);
        ontario.create_connections(vec![
            &alberta,
            &northwest_territory,
            &greenland,
            &quebec,
            &western_us,
            &eastern_us,
        ]);
        quebec.create_connections(vec![&ontario, &greenland, &eastern_us]);
        western_us.create_connections(vec![&alberta, &ontario, &eastern_us, &central_america]);
        eastern_us.create_connections(vec![&western_us, &central_america, &quebec, &ontario]);
        central_america.create_connections(vec![&western_us, &eastern_us, &venezuela]);

        venezuela.create_connections(vec![&central_america, &brazil, &peru]);
        brazil.create_connections(vec![&venezuela, &peru, &argentina, &north_africa]);
        peru.create_connections(vec![&venezuela, &brazil, &argentina]);
        argentina.create_connections(vec![&peru, &brazil]);

        iceland.create_connections(vec![&greenland, &scandinavia, &great_britain]);
        scandinavia.create_connections(vec![&iceland, &great_britain, &northern_europe, &ukraine]);
        great_britain.create_connections(vec![
            &iceland,
            &scandinavia,
            &northern_europe,
            &western_europe,
        ]);
        northern_europe.create_connections(vec![
            &great_britain,
            &scandinavia,
            &ukraine,
            &western_europe,
            &southern_europe,
        ]);
        ukraine.create_connections(vec![
            &scandinavia,
            &northern_europe,
            &southern_europe,
            &ural,
            &afghanistan,
            &middle_east,
        ]);
        western_europe.create_connections(vec![
            &north_africa,
            &great_britain,
            &northern_europe,
            &southern_europe,
        ]);
        southern_europe.create_connections(vec![
            &northern_europe,
            &ukraine,
            &western_europe,
            &middle_east,
            &north_africa,
            &egypt,
        ]);

        yakutsk.create_connections(vec![&siberia, &irkutsk, &kamchatka]);
        ural.create_connections(vec![&ukraine, &afghanistan, &china, &siberia]);
        siberia.create_connections(vec![
            &ural,
            &afghanistan,
            &china,
            &mongolia,
            &irkutsk,
            &yakutsk,
        ]);
        irkutsk.create_connections(vec![&siberia, &mongolia, &kamchatka, &yakutsk]);
        kamchatka.create_connections(vec![&yakutsk, &irkutsk, &mongolia, &japan, &alaska]);
        afghanistan.create_connections(vec![&ural, &ukraine, &middle_east, &india, &china]);
        china.create_connections(vec![
            &afghanistan,
            &india,
            &siam,
            &mongolia,
            &siberia,
            &ural,
        ]);
        mongolia.create_connections(vec![&china, &japan, &kamchatka, &irkutsk, &siberia]);
        japan.create_connections(vec![&mongolia, &kamchatka]);
        middle_east.create_connections(vec![
            &afghanistan,
            &ukraine,
            &southern_europe,
            &egypt,
            &east_africa,
            &india,
        ]);
        india.create_connections(vec![&middle_east, &siam, &china, &afghanistan]);
        siam.create_connections(vec![&china, &india, &indonesia]);

        north_africa.create_connections(vec![
            &western_europe,
            &southern_europe,
            &brazil,
            &congo,
            &east_africa,
            &egypt,
        ]);
        egypt.create_connections(vec![
            &north_africa,
            &east_africa,
            &middle_east,
            &southern_europe,
        ]);
        congo.create_connections(vec![&north_africa, &south_africa, &east_africa]);
        east_africa.create_connections(vec![
            &north_africa,
            &congo,
            &south_africa,
            &madagascar,
            &middle_east,
            &egypt,
        ]);
        south_africa.create_connections(vec![&congo, &east_africa, &madagascar]);
        madagascar.create_connections(vec![&south_africa, &east_africa]);

        indonesia.create_connections(vec![&siam, &new_guinea, &western_australia]);
        new_guinea.create_connections(vec![&indonesia, &western_australia, &eastern_australia]);
        western_australia.create_connections(vec![&indonesia, &new_guinea, &eastern_australia]);
        eastern_australia.create_connections(vec![&western_australia, &new_guinea]);

        boards::new(
            vec![
                &north_america,
                &europe,
                &asia,
                &south_america,
                &africa,
                &australia,
            ],
            vec![
                &alaska,
                &northwest_territory,
                &greenland,
                &alberta,
                &ontario,
                &quebec,
                &western_us,
                &eastern_us,
                &central_america,
                &venezuela,
                &brazil,
                &peru,
                &argentina,
                &iceland,
                &scandinavia,
                &great_britain,
                &northern_europe,
                &ukraine,
                &western_europe,
                &southern_europe,
                &yakutsk,
                &ural,
                &siberia,
                &irkutsk,
                &kamchatka,
                &afghanistan,
                &china,
                &mongolia,
                &japan,
                &middle_east,
                &india,
                &siam,
                &north_africa,
                &egypt,
                &congo,
                &east_africa,
                &south_africa,
                &madagascar,
                &indonesia,
                &new_guinea,
                &western_australia,
                &eastern_australia,
            ],
        )
    }
}

impl<'a> Board<'a> for ClassicBoard {
    fn claim_territory(&mut self, territory_index: usize, player: &Rc<PlayerStruct>) {
        boards::claim_territory(self, territory_index, player);
    }
}
