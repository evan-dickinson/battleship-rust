use crate::square::*;
use crate::board::*;
use crate::neighbor::*;

use smallvec::SmallVec;

pub fn place_ships_next_to_ends(board: &mut Board, changed: &mut bool) {
    let layout = board.layout;
    let ship_coords = layout.all_coordinates()
        .filter(|coord| { board[*coord].is_ship() })
        .collect::<SmallVec<[_; 20]>>();

    for coord in ship_coords {
        let neighbor = match board[coord] {
            Square::Ship(Ship::TopEnd)    => Neighbor::S,
            Square::Ship(Ship::BottomEnd) => Neighbor::N,
            Square::Ship(Ship::LeftEnd)   => Neighbor::E,
            Square::Ship(Ship::RightEnd)  => Neighbor::W,
            _                             => continue,
        };

        // Panic if neighbor is out of bounds. That would mean, for example, that there's the
        // top end of a ship on the last row of the board. There would be no place for the rest
        // of the ship to go.
        let neighbor_coord = layout.coord_for_neighbor(coord, neighbor).unwrap();
        if board[neighbor_coord] == Square::Unknown {
        	board.set(neighbor_coord, Square::Ship(Ship::Any), changed);
        }
    }        
}

#[cfg(test)]
mod test {
	use super::*;

	fn do_test(before: Vec<&str>, after: Vec<&str>) {
		let mut board = Board::new(&before);
		let expected = after.iter().map(|x| x.to_string()).collect::<Vec<_>>();

	    let mut _changed = false;
	    place_ships_next_to_ends(&mut board, &mut _changed);
	    assert_eq!(board.to_strings(), expected);        
	}

	#[test]
	fn it_places_ships_next_to_ends() {
	    do_test(vec![
	        "  00100",
	        "0|  ^  ",
	        "1|     ",
	    ],
		vec![
	        "  00000",
	        "0|  ^  ",
	        "0|  *  ",    
	    ]);
	}

	#[test]
	#[should_panic]
	fn it_panics_if_no_place_for_a_ship() {
	    let before = vec![
	        "  00100",
	        "1|  v  ",
	    ];

		let mut board = Board::new(&before);

	    let mut _changed = false;
	    place_ships_next_to_ends(&mut board, &mut _changed);
	}

	#[test]
	fn it_doesnt_overwrite_existing_ship() {
		// If it tries to overwrite v because it's adjacent to ^, board.set will assert.
		// Make sure that doesn't happen.
		do_test(vec![
	        "  00100",
	        "0|  ^  ",
	        "1|  v  ",	
	    ],
		vec![
	        "  00100",	
	        "0|  ^  ",
	        "1|  v  ",	
	   	]);
	}
}	
