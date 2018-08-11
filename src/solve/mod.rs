use square::*;
use neighbor::*;
use layout::*;
use board::*;


// TODO: Checks to implement:
// - Unify the "fill with water" functions
//   + Can handle all the separate checks with 1 algorithm
// - Convert "any" to specific ships:
//   - to dot, when fully surrounded
//   - to end, when surrounded by water and/or edge of board
//   - to vert middle, when surrounded by water on left/right
//   - to horz middle, when surrounded by water on top/bottom
//   - to generic middle, when surrounded by diagonals
//     + check for edge of board, too, not just surrounded by water

fn fill_with_water(board: &mut Board) {
    for row_or_col in board.layout.rows_and_cols() {
        if board.ships_remaining(row_or_col) == 0 {
            board.replace_unknown(row_or_col, Square::Water);
        }
    }
}

#[test]
fn it_fills_with_water() {
    let mut board = Board::new(vec![
        "  0011",
        "0|~*  ",
        "2|~*  ",
    ]);

    fill_with_water(&mut board);

    let result = board.to_strings();
    let expected = vec![
        "  0011".to_string(),
        "0|~*~~".to_string(),
        "2|~*  ".to_string(),       
    ];

    assert_eq!(result, expected);
}

// If number of Unknown squares on an axis == number of ships unaccounted for,
// fill the blank spots with ships
fn fill_with_ships(board: &mut Board) {
    for row_or_col in board.layout.rows_and_cols() {
        // Count unknown squares on this row or col
        let num_unknown = board.layout.coordinates(row_or_col)
            .filter(|coord| { board[*coord] == Square::Unknown } )
            .count();               

        if num_unknown == board.ships_remaining(row_or_col) {
            board.replace_unknown(row_or_col, Square::Ship(Ship::Any));
        }
    }
}

#[test]
fn it_fills_with_ships() {
    let mut board = Board::new(vec![
        "  0011",
        "0|~*~~",
        "2|~*  ",
    ]);

    fill_with_ships(&mut board);

    let expected = vec![
        "  0000".to_string(),
        "0|~*~~".to_string(),
        "0|~***".to_string(),               
    ];
    assert_eq!(board.to_strings(), expected);
}

fn surround_dots_with_water(board: &mut Board) {
    let layout = board.layout;
    let ship_coords = {
        layout.all_coordinates()
            .filter(|coord| { 
                board[*coord] == Square::Ship(Ship::Dot)
            })
            .collect::<Vec<_>>()
    };

    for coord in ship_coords {
        let neighbors = Neighbor::all_neighbors();
        let mut neighbor_coords = layout.coords_for_neighbors(coord, neighbors.iter());
        board.set_bulk(&mut neighbor_coords, Square::Water);
    }
}

#[test]
fn it_surrounds_dots() {
    let mut board = Board::new(vec![
        "  00000",
        "0|     ",
        "0|  •  ",
        "0|     ",
    ]);

    surround_dots_with_water(&mut board);
    let expected = vec![
        "  00000",
        "0| ~~~ ",
        "0| ~•~ ",
        "0| ~~~ ",
    ].iter().map(|x| x.to_string()).collect::<Vec<_>>();
    assert_eq!(board.to_strings(), expected);        
}

fn surround_ends_with_water(board: &mut Board) {
   let all_neighbors = Neighbor::all_neighbors();

    let ends = [
        // (a, b)
        // when you find a, fill in all neighbours except b
        (Ship::LeftEnd, Neighbor::E),
        (Ship::RightEnd, Neighbor::W),
        (Ship::TopEnd, Neighbor::S),
        (Ship::BottomEnd, Neighbor::N),
    ];

    let layout = board.layout;

    let neighbors_of_ships = ends.iter()
        // find all coords containing a ship of type end_type
        .map(|(end_type, ignore_neighbor)| {
            let nc = all_neighbors.clone();

            layout.all_coordinates()
                .filter(|coord| {
                    board[*coord] == Square::Ship(*end_type)
                })
                .map(|coord| {
                    let neighbors = nc.into_iter()
                        .filter(|&n| { *n != *ignore_neighbor });

                    layout.coords_for_neighbors(coord, neighbors)
                        .collect::<Vec<Coord>>()
                })
                .fold(vec![], move |mut acc, mut curr_vec| {
                    acc.append(&mut curr_vec);
                    acc
                })
        })
        .collect::<Vec<Vec<Coord>>>();

    for neighbor_coords in neighbors_of_ships {
        for coord in neighbor_coords {
            board.set(coord, Square::Water);
        }            
    }
}

fn surround_middles_with_water(board: &mut Board) {
    let layout = board.layout;
    let ship_coords = {
        layout.all_coordinates()
            .filter(|coord| { 
                board[*coord] == Square::Ship(Ship::VerticalMiddle) ||
                board[*coord] == Square::Ship(Ship::HorizontalMiddle)
            })
            .collect::<Vec<_>>()
    };

    for coord in ship_coords {
        let neighbors = match board[coord] {
            Square::Ship(Ship::VerticalMiddle) => [
                Neighbor::NE, Neighbor::E, Neighbor::SE,
                Neighbor::NW, Neighbor::W, Neighbor::SW,
            ],
            Square::Ship(Ship::HorizontalMiddle) => [
                Neighbor::NW, Neighbor::N, Neighbor::NE,
                Neighbor::SW, Neighbor::S, Neighbor::SE,
            ],
            _   => panic!("Should not happen"),
        };


        let mut neighbor_coords = layout.coords_for_neighbors(coord, neighbors.iter());
        board.set_bulk(&mut neighbor_coords, Square::Water);
    }
}


fn fill_diagonals_with_water(board: &mut Board) {
    let diagonals = [
        Neighbor::NE,
        Neighbor::SE,
        Neighbor::NW,
        Neighbor::SW
    ];

    let layout = board.layout;

    let ship_coords = {
        layout.all_coordinates()
            .filter(|coord| { board[*coord].is_ship() })
            .collect::<Vec<_>>()
    };
    for coord in ship_coords {
        let mut neighbor_coords = 
            layout.coords_for_neighbors(coord, diagonals.iter());

        board.set_bulk(&mut neighbor_coords, Square::Water);
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

    fill_diagonals_with_water (&mut board);
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

fn place_ships_next_to_ends(board: &mut Board) {
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

                board.set(neighbor_coord, Square::Ship(Ship::Any));
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

    place_ships_next_to_ends(&mut board);
    let expected = vec![
        "  00000",
        "0|  ^  ",
        "0|  *  ",    
    ].iter().map(|x| x.to_string()).collect::<Vec<_>>();
    assert_eq!(board.to_strings(), expected);
}



