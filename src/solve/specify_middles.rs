use square::*;
use board::*;
use neighbor::*;

// Convert an AnyMiddle to a specific type of middle
pub fn specify_middle(board: &mut Board, changed: &mut bool) {
    let layout = board.layout;
    let coords = layout.all_coordinates()
        .filter(|&coord| Square::Ship(Ship::AnyMiddle) == board[coord])
        .collect::<Vec<_>>();

    for coord in coords {
    	let vert_neighbors = [Neighbor::N, Neighbor::S];
    	let is_surrounded_vert = layout.coords_for_neighbors(coord, vert_neighbors.iter())
    		.all(|coord| board[coord] == Square::Water);

    	let horz_neighbors = [Neighbor::E, Neighbor::W];
    	let is_surrounded_horz = layout.coords_for_neighbors(coord, horz_neighbors.iter())
    		.all(|coord| board[coord] == Square::Water);    

    	assert_eq!(is_surrounded_vert && is_surrounded_horz, false);

  		// If we're surrounded vertically then this ship must be laid out horizontally,
  		// and vice versa.
    	if is_surrounded_vert {
    		board.set(coord, Square::Ship(Ship::HorizontalMiddle), changed);
    	}
    	else if is_surrounded_horz {
			board.set(coord, Square::Ship(Ship::VerticalMiddle), changed);
    	}
    }
}

#[cfg(test)]
mod test {
	use super::*;

	fn do_test(before: Vec<&str>, after: Vec<&str>) {
		let mut board = Board::new(before);
		let expected = after.iter().map(|x| x.to_string()).collect::<Vec<_>>();

	    let mut _changed = false;
	    specify_middle(&mut board, &mut _changed);
	    assert_eq!(board.to_strings(), expected);        
	}

	#[test]
	fn it_specifies_vertical_middle_surrounded_by_water() {
	    do_test(vec![
	        "  00200",
	        "1|     ",
	        "0| ~☐~ ",
	        "1|     ",
	    ],
		vec![
	        "  00200",
	        "1|     ",
	        "0| ~|~ ",
	        "1|     ", 
	    ]);
	}

	#[test]
	fn it_specifies_vertical_middle_at_edge_of_board() {
	    do_test(vec![
	        "  200",
	        "1|   ",
	        "0|☐~ ",
	        "1|   ",
	    ],
		vec![
	        "  200",
	        "1|   ",
	        "0||~ ",
	        "1|   ", 
	    ]);
	}	

	#[test]
	fn it_specifies_horizontal_middle_surrounded_by_water() {
	    do_test(vec![
	        "  01010",
	        "0|  ~  ",
	        "2|  ☐  ",
	        "0|  ~  ",
	    ],
		vec![
	        "  01010",
	        "0|  ~  ",
	        "2|  -  ",
	        "0|  ~  ",
	    ]);
	}	

	#[test]
	fn it_specifies_horizontal_middle_at_edge_of_board() {
	    do_test(vec![
	        "  01010",
	        "0|  ~  ",
	        "2|  ☐  ",
	    ],
		vec![
	        "  01010",
	        "0|  ~  ",
	        "2|  -  ",
	    ]);
	}		
}	