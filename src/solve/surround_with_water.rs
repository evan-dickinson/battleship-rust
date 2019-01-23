/////////////////////////////////////////////////////////////////////
//
// Solutions that surround ships

use crate::square::*;
use crate::board::*;

pub fn surround_ships_with_water(board: &mut Board, changed: &mut bool) {
    let layout = board.layout;
    let coords = layout.all_coordinates()
        .filter_map(|coord| { 
            match board[coord] {
                Square::Ship(ship_type) => Some((coord, ship_type)),
                _                       => None,
            }
        })
        .map(|(coord, ship_type)| {
            ship_type.water_neighbors()
                .into_iter()
                .filter_map(move |neighbor| coord.neighbor(neighbor))
        })
        .flatten()
        .collect::<Vec<_>>();

    for coord in coords {
        board.set(coord, Square::Water, changed);
    }
}


#[test]
fn it_fills_diagonals() {
    let mut board = Board::new(&vec![
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
    let mut board = Board::new(&vec![
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
    let mut board = Board::new(&vec![
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
    let mut board = Board::new(&vec![
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
