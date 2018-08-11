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

        let neighbors = Neighbor::surrounding_neighbors(ship_type);

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
