use square::*;
use neighbor::*;
use board::*;

mod fill_unknown;
use self::fill_unknown::*;

// TODO: Checks to implement:
// - Convert "any" to specific ships:
//   - to dot, when fully surrounded
//   - to end, when surrounded by water and/or edge of board
//   - to vert middle, when surrounded by water on left/right
//   - to horz middle, when surrounded by water on top/bottom
//   - to generic middle, when surrounded by diagonals
//     + check for edge of board, too, not just surrounded by water

fn surround_ships_with_water(board: &mut Board, changed: &mut bool) {
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

fn place_ships_next_to_ends(board: &mut Board, changed: &mut bool) {
    let layout = board.layout;
    let ship_coords = {
        layout.all_coordinates()
            .filter(|coord| { board[*coord].is_ship() })
            .collect::<Vec<_>>()
    };
    for coord in ship_coords {
        let neighbor = match board[coord] {
            Square::Ship(Ship::TopEnd) => Some(Neighbor::S),
            Square::Ship(Ship::BottomEnd) => Some(Neighbor::N),
            Square::Ship(Ship::LeftEnd) => Some(Neighbor::E),
            Square::Ship(Ship::RightEnd) => Some(Neighbor::W),
            _ => None,
        };

        if let Some(neighbor) = neighbor {
            if let Some(neighbor_coord) = 
                layout.coord_for_neighbor(coord, neighbor)  {

                board.set(neighbor_coord, Square::Ship(Ship::Any), changed);
            }
        }
    }        
}

#[test]
fn it_places_ships_next_to_ends() {
    let mut board = Board::new(vec![
        "  00100",
        "0|  ^  ",
        "1|     ",
    ]);

    let mut _changed = false;
    place_ships_next_to_ends(&mut board, &mut _changed);
    let expected = vec![
        "  00000",
        "0|  ^  ",
        "0|  *  ",    
    ].iter().map(|x| x.to_string()).collect::<Vec<_>>();
    assert_eq!(board.to_strings(), expected);
}

pub fn solve(board : &mut Board) {
    let solvers = [
        fill_with_water,
        fill_with_ships,
        surround_ships_with_water,
        place_ships_next_to_ends,
    ];

    board.print();
    loop {
        let mut changed_in_loop = false;

        for solve in solvers.iter() {
            let mut changed_in_step = false;
            solve(board, &mut changed_in_step);
            if changed_in_step {
                board.print();
            }

            changed_in_loop = changed_in_loop || changed_in_step;
        }

        if changed_in_loop == false {
            break;
        }
    }
}

