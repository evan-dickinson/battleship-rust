/////////////////////////////////////////////////////////////////////
//
// Solutions for the "only place it can go" rule

use square::*;
use board::*;
use layout::*;

pub fn find_only_place_for_ships(board: &mut Board, changed : &mut bool) {
	let sizes = board.remaining_ship_sizes().collect::<Vec<_>>();

	for ship_size in sizes {
		let num_ships = board.ships_to_find_for_size(ship_size);

		find_only_place_for_ship(board, ship_size, num_ships, changed);
	}
}

fn find_only_place_for_ship(board: &mut Board, ship_size: usize, num_ships: usize, changed: &mut bool) {
	// When placing size = 1, we don't increment the coordinate so axis doesn't matter. But if we
	// search by both axes, every coord will match twice. So only search by one axis, and we only match
	// every candidate coordinate once.
	let axes = if ship_size == 1 { 
		vec![Axis::Row]
	}
	else {
		vec![Axis::Row, Axis::Col]
	};

	let mut placements : Vec<(Coord, Axis)> = vec![];
	let layout = board.layout;

	for coord in layout.all_coordinates() {
		for incrementing_axis in axes.iter() {
			let constant_axis = incrementing_axis.cross_axis();

			if can_fit_ship_at_coord(board, ship_size, coord, *incrementing_axis) &&
				enough_free_ships_on_constant_axis(board, ship_size, coord, constant_axis) &&
				enough_free_ships_on_incrementing_axis(board, ship_size, coord, *incrementing_axis) {

				placements.push((coord, *incrementing_axis));
			}
		}
	}

	if placements.len() == num_ships {
		for (coord, incrementing_axis) in placements {
			place_ship_at_coord(board, ship_size, coord, incrementing_axis, changed);
		}
	}
}

// After can_place_ has returned true, actually place the ship
fn place_ship_at_coord(board: &mut Board, ship_size: usize, coord: Coord, incrementing_axis: Axis, changed: &mut bool) {
	for square_idx in 0..ship_size {
		let coord = board.layout.offset(coord, square_idx, incrementing_axis).unwrap();
		let new_value = Square::Ship(expected_square_for_ship(ship_size, square_idx, incrementing_axis));
		board.set(coord, new_value, changed);
	}
}

// constant axis: The one that remains the same as we increment through coordinats
// incrementing axis: The one that changes as we increment through coordinates
fn enough_free_ships_on_constant_axis(board: &Board, ship_size: usize, coord: Coord, constant_axis: Axis) -> bool {
	let row_or_col = RowOrCol {
		axis:  constant_axis,
		index: coord.index_for_axis(constant_axis),
	};	

	let ships_remaining = board.ships_remaining(row_or_col);
	return ships_remaining >= ship_size;
}

// In the incrementing axis, need to have one ship remaining per square
fn enough_free_ships_on_incrementing_axis(board: &Board, ship_size: usize, coord: Coord, incrementing_axis: Axis) -> bool {
	for square_idx in 0..ship_size {
		if let Some(coord) = board.layout.offset(coord, square_idx, incrementing_axis) {
			let row_or_col = coord.row_or_col(incrementing_axis);
			let ships_remaining = board.ships_remaining(row_or_col);
			if ships_remaining < 1 {
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
fn can_fit_ship_at_coord(board: &Board, ship_size: usize, coord: Coord, axis: Axis) -> bool {
	for square_idx in 0..ship_size {
		if let Some(coord) = board.layout.offset(coord, square_idx, axis) {

			let expeected = expected_square_for_ship(ship_size, square_idx, axis);
			let is_expected = 
				board[coord] == Square::Unknown || 
				board[coord] == Square::Ship(Ship::Any) ||
				board[coord] == Square::Ship(expeected);

			if !is_expected {
				return false;
			}
		}
		else {
			// Out of bounds
			return false;
		}
	}

	return true;
}

// Return the nth square for a ship, along the given axis.
// For example, a ship of size 3 on horizontal axis, we expect to see LeftEnd, then HorizontalMiddle, then RightEnd
fn expected_square_for_ship(ship_size: usize, square_idx: usize, incrementing_axis: Axis) -> Ship {
	assert!(square_idx < ship_size);

	if ship_size == 1 {
		return Ship::Dot;
	}
	else {
		if square_idx == 0 {
			return match incrementing_axis {
				// If we're incrementing columns, need to start with a left end.
				// If incrementing rows, need to start with a top end.
				Axis::Col => Ship::LeftEnd,
				Axis::Row => Ship::TopEnd,
			}
		}
		else if square_idx == ship_size - 1 {
			return match incrementing_axis {
				Axis::Col => Ship::RightEnd,
				Axis::Row => Ship::BottomEnd,
			}			
		}
		else { // middle
			return match incrementing_axis {
				Axis::Col => Ship::HorizontalMiddle,
				Axis::Row => Ship::VerticalMiddle,
			}
		}
	}
}

#[cfg(test)] 
mod test_only_place_it_can_go {
    use super::*;

    #[test]
    fn test_enough_free_ships_on_constant_axis() {
	    let board = Board::new(vec![
	        "  0000", // deliberate: Don't have enough ships on incrementing axis
	        "3|    ",
	    ]);
	    let coord = Coord { row_num: 0, col_num: 0 };
	    let result = enough_free_ships_on_constant_axis(&board, 3, coord, Axis::Row);
	    assert_eq!(result, true);

	    let result = enough_free_ships_on_constant_axis(&board, 4, coord, Axis::Row);
	    assert_eq!(result, false);
    }

    #[test]
    fn test_enough_free_ships_on_incrementing_axis() {
	    let board = Board::new(vec![
	        "  1110", 
	        "0|    ", // deliberate: Don't have enough ships on constant axis
	    ]);
	    let coord = Coord { row_num: 0, col_num: 0 };
	    let result = enough_free_ships_on_incrementing_axis(&board, 3, coord, Axis::Col);
	    assert_eq!(result, true);

	    let result = enough_free_ships_on_incrementing_axis(&board, 4, coord, Axis::Col);
	    assert_eq!(result, false);
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

	    // Can place it: All squares empty
	    let coord = Coord { row_num: 0, col_num: 0 };
	    let can_place = can_fit_ship_at_coord(&board, 3, coord, Axis::Col);
	    assert_eq!(can_place, true);

	    // Can place it: Just a dot
	    let coord = Coord { row_num: 0, col_num: 2 };
	    let can_place = can_fit_ship_at_coord(&board, 1, coord, Axis::Col);
	    assert_eq!(can_place, true);	    

	    // Cannot place it: Not enough room
	    let coord = Coord { row_num: 0, col_num: 0 };
	    let can_place = can_fit_ship_at_coord(&board, 5, coord, Axis::Col);
	    assert_eq!(can_place, false);

	    // Cannot place it: Water in the way
	    let coord = Coord { row_num: 1, col_num: 0 };
	    let can_place = can_fit_ship_at_coord(&board, 3, coord, Axis::Col);
	    assert_eq!(can_place, false);

	    // Can place it: Existing ships have the correct type
	    let coord = Coord { row_num: 1, col_num: 3 };
	    let can_place = can_fit_ship_at_coord(&board, 3, coord, Axis::Row);
	    assert_eq!(can_place, true);

	    // Cannot place it: Existing ship has the wrong type
	    let coord = Coord { row_num: 2, col_num: 0 };
	    let can_place = can_fit_ship_at_coord(&board, 1, coord, Axis::Row);
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
	    assert_eq!(board.to_strings(), expected);        
	}

	// TESTS:
	// - We know where the middle of the ship will be but not the ends. Place Ship::Any in the ones that we
	//   know will have a ship.
	//
	// TODO:
	// - After set, board needs to decrement number of ships found

	#[test]
	fn it_fills_in_4sq() {
	    do_test(vec![
	    	"ships: 4sq x 1.",
	        "  01111",
	        "4|~    ",
	    ],
	    vec![
	    	"ships: 4sq x 1.", // TODO: Should be 4sq x 0
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
	    	"ships: 4sq x 2.", // TODO: Should be 4sq x 0
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
	    	"ships: 2sq x 1.", // TODO: Should be 4sq x 0
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
	    	"ships: 3sq x 1.",
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
	    	"ships: 1sq x 1.", // TODO: Should be 1sq x 0
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
	    	"ships: 1sq x 2.", // TODO: Should be 1sq x 0
	        "  00000",
	        "0|~~~•~",
	        "0|~•~~~",

	    ]);
	}
}

