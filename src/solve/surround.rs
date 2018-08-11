/////////////////////////////////////////////////////////////////////
//
// Solutions that surround ships

use square::*;
use board::*;
use neighbor::*;

pub fn surround_ships_with_water(board: &mut Board, changed: &mut bool) {
    let layout = board.layout;
    let ship_coords = {
        layout.all_coordinates()
            .filter(|coord| { board[*coord].is_ship() })
            .collect::<Vec<_>>()
    };

    for coord in ship_coords {
        let ship_type = match board[coord] {
            Square::Ship(ship_type) => ship_type,
            _ => panic!("Unexpected"),
        };

        let neighbors = match ship_type {
            Ship::Any       => vec![
                Neighbor::NW, Neighbor::NE,
                Neighbor::SW, Neighbor::SE,
            ],

            Ship::Dot       => Neighbor::all_neighbors(),

            Ship::LeftEnd   => Neighbor::all_except(Neighbor::E),
            Ship::RightEnd  => Neighbor::all_except(Neighbor::W),
            Ship::TopEnd    => Neighbor::all_except(Neighbor::S),
            Ship::BottomEnd => Neighbor::all_except(Neighbor::N),

            Ship::VerticalMiddle => vec![
                Neighbor::NE, Neighbor::E, Neighbor::SE,
                Neighbor::NW, Neighbor::W, Neighbor::SW,
            ],
            Ship::HorizontalMiddle => vec![
                Neighbor::NW, Neighbor::N, Neighbor::NE,
                Neighbor::SW, Neighbor::S, Neighbor::SE,
            ],
        };


        let mut neighbor_coords = layout.coords_for_neighbors(coord, neighbors.iter());
        board.set_bulk(&mut neighbor_coords, Square::Water, changed);
    }
}


#[test]
fn it_fills_diagonals() {
    let mut board = Board::new(vec![
        "  00000",
        "0|     ",
        "0|     ",
        "0|  *  ",
        "0|     ",
        "0|     ",
    ]);

    let mut _changed = false;
    surround_ships_with_water(&mut board, &mut _changed);
    let expected = vec![
        "  00000",
        "0|     ",
        "0| ~ ~ ",
        "0|  *  ",
        "0| ~ ~ ",
        "0|     ",        
    ].iter().map(|x| x.to_string()).collect::<Vec<_>>();
    assert_eq!(board.to_strings(), expected);
}    

#[test]
fn it_surrounds_dots() {
    let mut board = Board::new(vec![
        "  00000",
        "0|     ",
        "0|  •  ",
        "0|     ",
    ]);

    let mut _changed = false;
    surround_ships_with_water(&mut board, &mut _changed);
    let expected = vec![
        "  00000",
        "0| ~~~ ",
        "0| ~•~ ",
        "0| ~~~ ",
    ].iter().map(|x| x.to_string()).collect::<Vec<_>>();
    assert_eq!(board.to_strings(), expected);        
}

#[test]
fn it_surrounds_middles() {
    let mut board = Board::new(vec![
        "  00000",
        "0|     ",
        "0|  -  ",
        "0|     ",
    ]);

    let mut _changed = false;
    surround_ships_with_water(&mut board, &mut _changed);
    let expected = vec![
        "  00000",
        "0| ~~~ ",
        "0|  -  ",
        "0| ~~~ ",
    ].iter().map(|x| x.to_string()).collect::<Vec<_>>();
    assert_eq!(board.to_strings(), expected);        
}

#[test]
fn it_surrounds_ends() {
    let mut board = Board::new(vec![
        "  00000",
        "0|     ",
        "0|  ^  ",
        "0|     ",
    ]);

    let mut _changed = false;
    surround_ships_with_water(&mut board, &mut _changed);
    let expected = vec![
        "  00000",
        "0| ~~~ ",
        "0| ~^~ ",
        "0| ~ ~ ",
    ].iter().map(|x| x.to_string()).collect::<Vec<_>>();
    assert_eq!(board.to_strings(), expected);        
}
