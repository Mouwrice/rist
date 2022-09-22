//! The original classic Risk board
use std::rc::Rc;

use colored::Colorize;

use crate::boards::BoardStruct;
use crate::boards::BoardType::ClassicBoard;
use crate::continent::Continent;
use crate::territory::Territory;

pub fn new(players: usize) -> BoardStruct {
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
        "Northwest Territory",
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

    BoardStruct::generate_board(
        ClassicBoard,
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
            &yakutsk,
            &alberta,
            &ontario,
            &quebec,
            &iceland,
            &scandinavia,
            &ural,
            &siberia,
            &irkutsk,
            &kamchatka,
            &western_us,
            &eastern_us,
            &great_britain,
            &northern_europe,
            &ukraine,
            &afghanistan,
            &china,
            &mongolia,
            &japan,
            &central_america,
            &western_europe,
            &southern_europe,
            &middle_east,
            &india,
            &siam,
            &venezuela,
            &brazil,
            &north_africa,
            &egypt,
            &indonesia,
            &new_guinea,
            &peru,
            &argentina,
            &congo,
            &east_africa,
            &western_australia,
            &eastern_australia,
            &south_africa,
            &madagascar,
        ],
        6,
    )
}

pub fn print_board(board: &BoardStruct) {
    // Armies on the territory
    let mut a = vec![];
    // Territory names
    let mut n = vec![];
    // Surrounding star coloring
    let mut s = vec![];
    for territory in &board.territories {
        let player = territory.get_player();
        if player.is_none() {
            a.push(format!("* {:05} *", 0).white());
            n.push(format!("* {:5} *", territory.abbr).white());
            s.push("*******".white());
        } else if let Some(player) = territory.get_player() {
            n.push(player.colorize(format!("* {:5} *", territory.abbr)));
            a.push(player.colorize(format!("* {:05} *", territory.armies.borrow())));
            s.push(player.colorize(String::from("*******")));
        }
    }

    // Extra info
    let mut i = vec![];
    for index in 0..6 {
        let mut info = match board.extra_info.borrow().get(index) {
            Some(text) => String::from(text),
            None => String::from(""),
        };

        if info.len() > 74 {
            info.truncate(74);
            info.replace_range(70..74, ".");
        }
        i.push(format!("{:<74}", info));
    }

    print!(
        r"         +-----------------------------------------------------------------------------------------------------------------------------------------------------------------------+
         |                                                                                                                                                                       |
  # # #  |  # # # # # # # # # # # # # # # # # # #                                                                                                        # # # # # # # # # # #   |
 #       |                                       #                                                                                                      #                     #  |
#     {}         {}         {}     #                                                                                                    #      {}            |
#    {} ----- {} ----- {} -----+                  ..........                                               ........              #      {}           |  #
#    {}       {}       {}    #  \                 . EUROPE .                                               . ASIA .             #       {}           |    #
#     {}      /  {}      /  {}     #   \                ..........                                               ........            #      /  {}  \         |     #
#        |        /      |        /      |        #    \                                                                                           #      /      |      \        |      #
#        |       /       |       /       |        #     \     # # # # # # # # # # # # # # # #                             # # # # # # # # # # # # #      /       |       \       |       #
#        |      /        |      /        |        #      \   #                               #                           #                              /        |        \      |       #
#     {}  /      {}  /      {}     #       \ #     {}         {}     #                          #    {}         {}  /      {}      \  {}    #
#    {} ----- {} ----- {}    #        ----- {} ----- {}     #                      +----- {} ----- {} ----- {} ----- {}   #
#    {}       {}       {}    #         #    {}       {}      #                    /   #   {}       {}       {}       {}   #
#     {}      /  {}      /  {}     #         #     {}      /  {}  \     #                  /    #    {}  \      {}  \      {}      /  {}    #
#        |        /      |        /              #          #        |        /      |      \     #                /     #       |      \        |      \        |        /      |       #
#        |       /       |       /     # # # # #            #        |       /       |       \     # # # # #      /      #       |       \       |       \       |       /       |       #
#        |      /        |      /     #                     #        |      /        |        \             #    /       #       |        \      |        \      |      /        |       #
#     {}  /      {}  /     #                      #     {}  /      {}      \  {}    #  /        #    {}      \  {}      \  {}  /      {}    #
#    {} ----- {}      #                       #    {} ----- {} ----- {} ----+------------ {} ----- {} ----- {} ----- {}   #
#    {}       {}     #                        #    {}       {}       {}    # \        #   {}       {}       {}       {}   #
#     {}      /  {}     #                         #     {}  \      {}  \      {}     #  \       #    {}  \      {}  \      {}         {}    #
#        |        /              #                          #               \        |      \        |        #   \      #       |      \        |      \        |                      #
#        |       /     # # # # #                             #               \       |       \       |        #    \     #       |       \       |       \       |           # # # # # #
#        |      /     #                                       #               \      |        \      |        #     \    #       |        \      |        \      |          #
#     {}  /     #   .................                     #               \  {}      \  {}     #      \   #    {}      \  {}      \  {}      #
#    {}      #    . NORTH AMERICA .                      #                {} ----- {} -----------+----- {} ----- {} ----- {}    #
#    {}     #     .................                       #               {}       {}    #      /   #   {}       {}       {}   #
#     {}     #                                               #               {}       / {}     #     /      /  {}         {}         {}   #
 #       |       #                                                 #                 |         /     |      #      /      /                                      |     #
  # # #  |  # # #                                                   # # # # # # # #  |  # #   /   #  |  # #       /      /  # # # # # # # # # # # # # # # # # #  |  # #
         |                                                                           |       /       |           /      /                                        |
  # # #  |  # # # # # # # # # # #                                             # # #  |  #   /  # #   |  # # #   /      /                                  # # #  | # # # # # # # # # # # #
 #       |                       #                                           #       |     /         |       # /      /                                 #        |                        #
#     {}         {}     #                                         #     {} /       {}     /      /                                  #     {}         {}     #
#    {} ----- {} ------------------------------------------------- {} ----- {} --+      /                                   #    {} ----- {}    #
#    {}       {}    #                                         #    {}       {}    #    /                   .............    #    {}       {}    #
#     {}      /  {}     #                                         #     {}  \      {}     #   /                    . AUSTRALIA .    #     {}      /  {}     #
#        |        /      |        #                                         #        |      \        |        #  /                     .............    #        |        /      |        #
#        |       /       |        #                                         #        |       \       |        # /                                       #        |       /       |        #
#        |      /        |        #                                         #        |        \      |         /                                        #        |      /        |        #
#     {}  /      {}     #   .................                     #     {}      \  {}     /                                         #      {}  /      {}    #
#    {} ----- {}    #   . SOUTH AMERICA .                     #    {} ----- {} --+                                          #     {} ----- {}   #
#    {}       {}    #   .................                     #    {}       {}    #                                         #     {}       {}   #
#     {}         {}     #                                         #     {}      /  {}     #                                         #      {}         {}    #
 #                               #                                          #        |        /      |        #                                         #                                 #
  # # # # # # # # # # # # # # # #                                           #        |       /       |        #                                           # # # # # # # # # # # # # # # #
                                                                            #        |      /        |        #
 {} #     {}  /      {}     #   ..........
 {} #    {} ----- {}    #   . AFRICA .
 {} #    {}       {}    #   ..........
 {} #     {}         {}     #
 {}  #                               #
 {}    # # # # # # # # # # # # # # #
",
        s[0],
        s[1],
        s[2],
        s[3],
        n[0],
        n[1],
        n[2],
        n[3],
        a[0],
        a[1],
        a[2],
        a[3],
        s[0],
        s[1],
        s[2],
        s[3],
        s[4],
        s[5],
        s[6],
        s[7],
        s[8],
        s[9],
        s[10],
        s[11],
        s[12],
        n[4],
        n[5],
        n[6],
        n[7],
        n[8],
        n[9],
        n[10],
        n[11],
        n[12],
        a[4],
        a[5],
        a[6],
        a[7],
        a[8],
        a[9],
        a[10],
        a[11],
        a[12],
        s[4],
        s[5],
        s[6],
        s[7],
        s[8],
        s[9],
        s[10],
        s[11],
        s[12],
        s[13],
        s[14],
        s[15],
        s[16],
        s[17],
        s[18],
        s[19],
        s[20],
        s[21],
        n[13],
        n[14],
        n[15],
        n[16],
        n[17],
        n[18],
        n[19],
        n[20],
        n[21],
        a[13],
        a[14],
        a[15],
        a[16],
        a[17],
        a[18],
        a[19],
        a[20],
        a[21],
        s[13],
        s[14],
        s[15],
        s[16],
        s[17],
        s[18],
        s[19],
        s[20],
        s[21],
        s[22],
        s[23],
        s[24],
        s[25],
        s[26],
        s[27],
        n[22],
        n[23],
        n[24],
        n[25],
        n[26],
        n[27],
        a[22],
        a[23],
        a[24],
        a[25],
        a[26],
        a[27],
        s[22],
        s[23],
        s[24],
        s[25],
        s[26],
        s[27],
        s[28],
        s[29],
        s[30],
        s[31],
        s[32],
        s[33],
        n[28],
        n[29],
        n[30],
        n[31],
        n[32],
        n[33],
        a[28],
        a[29],
        a[30],
        a[31],
        a[32],
        a[33],
        s[28],
        s[29],
        s[30],
        s[31],
        s[32],
        s[33],
        s[34],
        s[35],
        s[36],
        s[37],
        s[38],
        s[39],
        n[34],
        n[35],
        n[36],
        n[37],
        n[38],
        n[39],
        a[34],
        a[35],
        a[36],
        a[37],
        a[38],
        a[39],
        s[34],
        s[35],
        s[36],
        s[37],
        s[38],
        s[39],
        i[0],
        s[40],
        s[41],
        i[1],
        n[40],
        n[41],
        i[2],
        a[40],
        a[41],
        i[3],
        s[40],
        s[41],
        i[4],
        i[5]
    )
}
