/////////////////////////////////////////////////////////////////////
//
// Solutions for the "only place it can go" rule

use square::*;
use board::*;
use layout::*;

use std::collections::HashSet;

pub fn find_only_place_for_ships(board: &mut Board, changed : &mut bool) {
	let sizes = board.remaining_ship_sizes().collect::<Vec<_>>();

	for ship_size in sizes {
		let num_ships = board.ships_to_find_for_size(ship_size);

		find_only_place_for_ship(board, ship_size, num_ships, changed);
	}
}

fn find_only_place_for_ship(board: &mut Board, ship_size: usize, num_ships: usize, changed: &mut bool) {
	let mut placements : Vec<(Coord, Axis)> = vec![];

	board.iterate_possible_ships(ship_size, |coord, incrementing_axis| {
		let constant_axis = incrementing_axis.cross_axis();

		let (can_fit, num_ship_squares) = can_fit_ship_at_coord(board, ship_size, coord, incrementing_axis);

		if  can_fit &&
			enough_free_ships_on_constant_axis(board, ship_size, coord, constant_axis, num_ship_squares) &&
			enough_free_ships_on_incrementing_axis(board, ship_size, coord, incrementing_axis) {

			placements.push((coord, incrementing_axis));
		}
	});

	if placements.len() == num_ships {
		// We know the placement of every square in the ships. Fill in the complete ships.

		for (coord, incrementing_axis) in placements {
			place_ship_at_coord(board, ship_size, coord, incrementing_axis, changed);
		}
	}
	// TODO: Handle cases where there are multiple ships remaining, each with a fuzzy place
	else if num_ships == 1 {
		// We can't fill in *all* the squares of the ships, but perhaps we can fill in *some*.
		// Example: We need to fill a ship of size 4 and there are 5 possible squares. We know
		// the middle 3 squares will be ships, even if we don't know which square will hold the 4th
		// ship.

		let layout = board.layout;

		// For each placement, create a HashSet of coordinates in that placement
		let mut coordinates_for_all_placements = placements.iter().map(|(coord, incrementing_axis)| {
			let all_coords_in_this_placement = (0..ship_size).map(|square_idx| {
				layout.offset(*coord, square_idx, *incrementing_axis).unwrap()
			}).collect::<HashSet<_>>();

			all_coords_in_this_placement
		});

		place_ship_at_intersection_of_coords(board, &mut coordinates_for_all_placements, num_ships, changed);
	}
}

// After determining we can place a ship here, place it.
fn place_ship_at_coord(board: &mut Board, ship_size: usize, coord: Coord, incrementing_axis: Axis, changed: &mut bool) {
	for square_idx in 0..ship_size {
		let coord = board.layout.offset(coord, square_idx, incrementing_axis).unwrap();
		let new_value = Square::Ship(Ship::expected_square_for_ship(ship_size, square_idx, incrementing_axis));
		board.set(coord, new_value, changed);
	}
}

fn place_ship_at_intersection_of_coords(board: &mut Board, coordinates_for_all_placements: &mut impl Iterator<Item = HashSet<Coord>>, 
	num_ships: usize, changed: &mut bool) {

	if let Some(coords_for_first_placement) = coordinates_for_all_placements.next() {
		// Find the intersection of all the hash sets
		let common_coordinates = coordinates_for_all_placements.fold(coords_for_first_placement, |acc, coords_for_placement| {
			acc.intersection(&coords_for_placement).cloned().collect::<HashSet<_>>()
		});

		// Set coordinates in the intersection
		for coord in common_coordinates {
			if board[coord] == Square::Unknown {
				board.set(coord, Square::Ship(Ship::Any), changed);
			}
		}
	}	
}

// constant axis: The one that remains the same as we increment through coordinats
// incrementing axis: The one that changes as we increment through coordinates
fn enough_free_ships_on_constant_axis(board: &Board, ship_size: usize, coord: Coord, constant_axis: Axis, num_ship_squares: usize) -> bool {
	let row_or_col = RowOrCol {
		axis:  constant_axis,
		index: coord.index_for_axis(constant_axis),
	};	

	let ships_remaining = board.ships_remaining(row_or_col);
	return ships_remaining >= ship_size - num_ship_squares;
}

// In the incrementing axis, need to have one ship remaining per square
fn enough_free_ships_on_incrementing_axis(board: &Board, ship_size: usize, coord: Coord, incrementing_axis: Axis) -> bool {
	for square_idx in 0..ship_size {
		if let Some(coord) = board.layout.offset(coord, square_idx, incrementing_axis) {
			let row_or_col = coord.row_or_col(incrementing_axis);
			let ships_remaining = board.ships_remaining(row_or_col);
			if ships_remaining < 1 && !board[coord].is_ship() {
				return false;
			}
		}
		else {
			return false;
		}
	}

	return true;
}

// Will the ship fit on the board at the given coordinates?
//
// Return:
// bool - Will the ship fit?
// usize - Number of ship squares already placed (only correct if bool is true)
fn can_fit_ship_at_coord(board: &Board, ship_size: usize, coord: Coord, incrementing_axis: Axis) -> (bool, usize) {
	let mut num_ship_squares = 0;
	let mut all_matches_exact = true;

	let fits = board.test_ship_at_coord(ship_size, coord, incrementing_axis,
		|coord, square_idx| {
			if board[coord].is_ship() {
				num_ship_squares += 1;
			}

			let expected = Ship::expected_square_for_ship(ship_size, square_idx, incrementing_axis);
			
			let is_exact_match =  board[coord] == Square::Ship(expected);
			let is_match = is_exact_match ||
				board[coord] == Square::Unknown || 
				board[coord] == Square::Ship(Ship::Any) ||
				(
					board[coord] == Square::Ship(Ship::AnyMiddle) &&
					(expected == Ship::VerticalMiddle || expected == Ship::HorizontalMiddle)
				);				

			all_matches_exact = all_matches_exact && is_exact_match;

			is_match
		});

	return (fits && !all_matches_exact, num_ship_squares);
}

#[cfg(test)] 
mod test_only_place_it_can_go {
    use super::*;

    #[test]
    fn test_enough_free_ships_on_constant_axis() {
	    let board = Board::new(vec![
	        "  0000", // deliberate: Don't have enough ships on incrementing axis
	        "3|    ",
	        "0|    ",
	        "2| *  ",
	    ]);

	    // Enough space - No existing ships
	    let coord = Coord { row_num: 0, col_num: 0 };
	    let result = enough_free_ships_on_constant_axis(&board, 3, coord, Axis::Row, 0);
	    assert_eq!(result, true);

	    // Enough space - Includes an existing ships
	    let coord = Coord { row_num: 2, col_num: 0 };
	    let result = enough_free_ships_on_constant_axis(&board, 3, coord, Axis::Row, 1);
	    assert_eq!(result, true);	 

	    // Not enough space
	    let coord = Coord { row_num: 0, col_num: 0 };
	    let result = enough_free_ships_on_constant_axis(&board, 4, coord, Axis::Row, 0);
	    assert_eq!(result, false);   
    }

    #[test]
    fn test_enough_free_ships_on_incrementing_axis() {
	    let board = Board::new(vec![
	        "  1110", 
	        "0|    ", // deliberate: Don't have enough ships on constant axis
	    ]);

	    // Enough space - No existing ships
	    let coord = Coord { row_num: 0, col_num: 0 };
	    let result = enough_free_ships_on_incrementing_axis(&board, 3, coord, Axis::Col);
	    assert_eq!(result, true);

	    // Not enough space
	    let coord = Coord { row_num: 0, col_num: 0 };	    
	    let result = enough_free_ships_on_incrementing_axis(&board, 4, coord, Axis::Col);
	    assert_eq!(result, false);

	    let board = Board::new(vec![
	        "  1010", 
	        "0| *  ", // deliberate: Don't have enough ships on constant axis
	    ]);	  

	    // Enough space - Includes an existing ship
	    let coord = Coord { row_num: 0, col_num: 0 };
	    let result = enough_free_ships_on_incrementing_axis(&board, 3, coord, Axis::Col);
	    assert_eq!(result, true);	      

    }    

    #[test]
    fn test_can_fit_ship_at_coord() {
	    let board = Board::new(vec![
	        "  0000",
	        "0|    ",
	        "0|~ ~ ",
	        "0|< ~*",
	        "0|~~~v",
	    ]);    	

	    // Can place it: All squares empty (non-dot ship)
	    let coord = Coord { row_num: 0, col_num: 0 };
	    let (can_place, num_ship_squares) = can_fit_ship_at_coord(&board, 3, coord, Axis::Col);
	    assert_eq!(can_place, true);
	    assert_eq!(num_ship_squares, 0);

	    // Can place it: All squares empty (dot ship)
	    let coord = Coord { row_num: 0, col_num: 2 };
	    let (can_place, num_ship_squares) = can_fit_ship_at_coord(&board, 1, coord, Axis::Col);
	    assert_eq!(can_place, true);	
	    assert_eq!(num_ship_squares, 0);    

	    // Cannot place it: Not enough room
	    let coord = Coord { row_num: 0, col_num: 0 };
	    let (can_place, _) = can_fit_ship_at_coord(&board, 5, coord, Axis::Col);
	    assert_eq!(can_place, false);

	    // Cannot place it: Water in the way
	    let coord = Coord { row_num: 1, col_num: 0 };
	    let (can_place, _) = can_fit_ship_at_coord(&board, 3, coord, Axis::Col);
	    assert_eq!(can_place, false);

	    // Can place it: Existing ships have the correct type
	    let coord = Coord { row_num: 1, col_num: 3 };
	    let (can_place, num_ship_squares) = can_fit_ship_at_coord(&board, 3, coord, Axis::Row);
	    assert_eq!(can_place, true);
	    assert_eq!(num_ship_squares, 2);

	    // Cannot place it: Existing ship has the wrong type
	    let coord = Coord { row_num: 2, col_num: 0 };
	    let (can_place, _) = can_fit_ship_at_coord(&board, 1, coord, Axis::Row);
	    assert_eq!(can_place, false);		    
    }

    #[test]
    fn test_can_fit_ship_doesnt_count_completed_ships() {
	    let board = Board::new(vec![
	        "  1111",
	        "1|<-->",
	    ]);

	    // Cannot place it: The entire ship is present
	    let coord = Coord { row_num: 0, col_num: 0 };
	    let (can_place, _) = can_fit_ship_at_coord(&board, 4, coord, Axis::Col);
	    assert_eq!(can_place, false);
    }

    #[test]
    fn test_place_ship_at_coord() {
	    let mut board = Board::new(vec![
	        "  002",
	        "1|   ",
	        "1| ~ ",
	        "0|• *",
	        "0|  v",
	    ]);        

	    let mut changed = false;
	    let coord = Coord { row_num: 0, col_num: 2 };
	    place_ship_at_coord(&mut board, 4,  coord, Axis::Row, &mut changed);

	    assert_eq!(true, changed);

	    let expected = vec![
	        "  000",
	        "0|  ^",
	        "0| ~|",
	        "0|• |",
	        "0|  v",
	    ].iter().map(|x| x.to_string()).collect::<Vec<_>>();

		assert_eq!(board.to_strings(), expected); 
	}

	fn do_test(before: Vec<&str>, after: Vec<&str>) {
		let mut board = Board::new(before);
		let expected = after.iter().map(|x| x.to_string()).collect::<Vec<_>>();

	    let mut _changed = false;
	    find_only_place_for_ships(&mut board, &mut _changed);

	    let board_strings = board.to_strings();

	    assert_eq!(board_strings, expected);

	    // assert_eq!(board_strings.len(), expected.len());

	    // let text_lines = board_strings.iter().zip(expected.iter());
	    // for (actual_line, expected_line) in text_lines {
	    // 	assert_eq!(actual_line, expected_line);
	    // }
	}

	// TESTS:
	// - We know where the middle of the ship will be but not the ends. Place Ship::Any in the ones that we
	//   know will have a ship.

	#[test]
	fn it_fills_in_4sq() {
	    do_test(vec![
	    	"ships: 4sq x 1.",
	        "  01111",
	        "4|~    ",
	    ],
	    vec![
	    	"ships: 4sq x 0.",
	        "  00000",
	        "0|~<-->",
	    ]);
	}	

	#[test]
	fn it_fills_in_4sq_x2() {
	    do_test(vec![
	    	"ships: 4sq x 2.",
	        "  02222",
	        "4|~    ",
	        "0|~~~~~",
	        "4|~    ",
	    ],
	    vec![
	    	"ships: 4sq x 0.",
	        "  00000",
	        "0|~<-->",
	        "0|~~~~~",
	        "0|~<-->",
	    ]);
	}	

	#[test]
	fn it_doesnt_fill_if_not_enough_space() {
	    do_test(vec![
	    	"ships: 4sq x 1.",
	        "  01111",
	        "3|~    ", // Only 3 ships to place here. Not enough room for the 4sq ship.
	    ],
	    vec![
	    	"ships: 4sq x 1.",
	        "  01111",
	        "3|~    ",
	    ]);
	}

	#[test]
	fn it_fills_only_where_there_is_space_on_incrementing_axis() {
	    do_test(vec![
	    	"ships: 2sq x 1.",
	        "  00110",
	        "2|~    ",
	    ],
	    vec![
	    	"ships: 2sq x 0.", 
	        "  00000",
	        "0|~ <> ", // These middle squares were the only ones with space on incrementing axis
	    ]);
	}	

	#[test]
	fn it_fills_only_where_there_is_space_on_constant_axis() {
	    do_test(vec![
	    	"ships: 3sq x 1.",
	        "  030",
	        "2|   ",
	        "2|   ",
	        "2|   ",
	    ],
	    vec![
	    	"ships: 3sq x 0.",
	        "  000",
	        "1| ^ ",
	        "1| | ",
	        "1| v ",
	    ]);
	}		

	#[test]
	fn it_fills_in_dot() {
	    do_test(vec![
	    	"ships: 1sq x 1.",
	        "  00010",
	        "1|~~~ ~",
	    ],
	    vec![
	    	"ships: 1sq x 0.",
	        "  00000",
	        "0|~~~•~",
	    ]);
	}	

	#[test]
	fn it_fills_in_2_dots() {
	    do_test(vec![
	    	"ships: 1sq x 2.",
	        "  01010",
	        "1|~~~ ~",
	        "1|~ ~~~",
	    ],
	    vec![
	    	"ships: 1sq x 0.",
	        "  00000",
	        "0|~~~•~",
	        "0|~•~~~",

	    ]);
	}

	#[test]
	fn it_completes_partial_ship() {
	    do_test(vec![
	    	"ships: 5sq x 1.",
	        "  020",
	        "1|~ ~",
	        "0|~*~",
	        "1|~ ~",
	        "0|~*~",
	        "0|~*~",	        
	        "0|~ ~",
	    ],
	    vec![
	    	"ships: 5sq x 0.",
	        "  000",
	        "0|~^~",
	        "0|~|~",
	        "0|~|~",
	        "0|~|~",
	        "0|~v~",	        
	        "0|~ ~",	    
	    ]);
	}

	#[test]
	fn it_completes_generic_middle_horizontally() {
	    do_test(vec![
	    	"ships: 3sq x 1.",
	    	"  01010",
	    	"2|  ☐  ",
	    ],
	    vec![
	    	"ships: 3sq x 0.",
	    	"  00000",
	    	"0| <-> ",
	    ]);
	}	

	#[test]
	fn it_completes_generic_middle_vertically() {
	    do_test(vec![
	    	"ships: 3sq x 1.",
	    	"  020",
	    	"0|   ",
	    	"1|   ",
	    	"0| ☐ ",
	    	"1|   ",
	    	"0|   ",
	    ],
	    vec![
	    	"ships: 3sq x 0.",
	    	"  000",
	    	"0|   ",
	    	"0| ^ ",
	    	"0| | ",
	    	"0| v ",
	    	"0|   ",
	    ]);
	}		

	#[test]
	fn it_places_partial_ship_when_one_possibility() {
	    do_test(vec![
	    	"ships: 5sq x 1.",
	    	"  1111111",
	    	"7|       ",
	    ],
	    vec![
	    	"ships: 5sq x 1.",
	    	"  1100011",
	    	"4|  ***  ",
	    ]);		
	}

	#[test]
	fn it_doesnt_place_partial_ship_when_two_possibilities() {
	    do_test(vec![
	    	"ships: 5sq x 1.",
	    	"  1111111",
	    	"7|       ",
	    	"7|       ",
	    ],
	    vec![
	    	"ships: 5sq x 1.",
	    	"  1111111",
	    	"7|       ",
	    	"7|       ",
	    ]);		
	}	
}

