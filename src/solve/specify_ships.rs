/////////////////////////////////////////////////////////////////////
//
// Convert "any" ships to specific ships

use std::collections::HashSet;

use square::*;
use board::*;
use neighbor::*;
use layout::*;

// Return true if all neighbors are either water or they are out of bounds
fn is_water_or_out_of_bounds<'a>(board: &'a Board, index: Coord, neighbors: impl IntoIterator<Item = &'a Neighbor> + 'a) -> bool {
    let mut neighbor_coords = board.layout.coords_for_neighbors(index, neighbors);

    // coords_for_neighbors filterd out coords that are out of bounds. 
    // Now check to see that all remaining neighbors are water.

    return neighbor_coords.all(|coord| board[coord] == Square::Water);
}

// Return true if all neighbors are in bounds and they are all ships
fn is_ship<'a>(board: &'a Board, index: Coord, neighbors: impl IntoIterator<Item = &'a Neighbor> + 'a) -> bool {
	// Can't use layout.coords_for_neighbors here because that filters out neigbors that are out of bounds

    return neighbors.into_iter().all(|&neighbor| {
    	if let Some(coord) = board.layout.coord_for_neighbor(index, neighbor) {
    		return board[coord].is_ship();
    	}
    	else {
    		return false;
    	}
    });

}

pub fn specify_ships(board: &mut Board, changed: &mut bool) {
    let layout = board.layout;
    let ship_coords = {
        layout.all_coordinates()
            .filter(|coord| { board[*coord] == Square::Ship(Ship::Any) })
            .collect::<Vec<_>>()
    };

    for coord in ship_coords {
    	// Find the ship type that will update the most neighboring squares
    	let mut neighbor_count : usize = 0;
    	let mut new_value = None;

    	for ship_type in Ship::all() {
    		let all_neighbors = Neighbor::all_neighbors();
    		let surrounding_neighbors = Neighbor::surrounding_neighbors(ship_type);
    		let empty_neighbors = all_neighbors.difference(&surrounding_neighbors);

	    	if is_water_or_out_of_bounds(board, coord, surrounding_neighbors.iter()) &&
	    	   is_ship(board, coord, empty_neighbors) &&
	    		surrounding_neighbors.len() > neighbor_count {

	    		new_value = Some(ship_type);
	    		neighbor_count = surrounding_neighbors.len();
	    	}
    	}

    	if let Some(ship_type) = new_value {
    		board.set(coord, Square::Ship(ship_type), changed);
    		assert_eq!(*changed, true);
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
	    specify_ships(&mut board, &mut _changed);
	    assert_eq!(board.to_strings(), expected);        
	}

	#[test]
	fn it_creates_dot_surrounded_by_water() {
	    do_test(vec![
	        "  00000",
	        "0| ~~~ ",
	        "0| ~*~ ",
	        "0| ~~~ ",
	    ],
	    vec![
	        "  00000",
	        "0| ~~~ ",
	        "0| ~•~ ",
	        "0| ~~~ ",
	    ]);
	}

	#[test]
	fn it_creates_dot_in_corner() {
	    do_test(vec![
	        "  000",
	        "0| ~~",
	        "0| ~*",
	    ],
	    vec![
	        "  000",
	        "0| ~~",
	        "0| ~•",
	    ]);        
	}	

	#[test]
	fn it_doesnt_create_dot_without_water_north() {
	    do_test(vec![
	        "  000",
	        "0| ~ ",
	        "0| ~*",
	    ],
	    vec![
	        "  000",
	        "0| ~ ", 
	        "0| ~*", // no change to dot, because north neighbor is unknown
	    ]);        
	}		

	#[test]
	fn it_doesnt_create_dot_without_water_west() {
	    do_test(vec![
	        "  000",
	        "0| ~~",
	        "0|  *",
	    ],
	    vec![
	        "  000",
	        "0| ~~", 
	        "0|  *", // no change to dot, because west neighbor is unknown
	    ]);        
	}	

	#[test]
	fn it_creates_left_end_away_from_border() {
	    do_test(vec![
	        "  00000",
	        "0|~~~~ ",
	        "0|~*-> ",
	        "0|~~~~ ",
	    ],
	    vec![
	        "  00000",
	        "0|~~~~ ",
	        "0|~<-> ",
	        "0|~~~~ ",
	    ]);
	}	

	#[test]
	fn it_creates_left_end_at_border() {
		do_test(vec![
	        "  000",
	        "0|~~~",
	        "0|*> ",
	        "0|~~~",	        
	    ],
	    vec![
	        "  000",
	        "0|~~~",
	        "0|<> ",
	        "0|~~~",
	    ]);   
	}

	#[test]
	fn it_creates_left_end_in_corner() {
		do_test(vec![
	        "  000",
	        "0|*> ",
	        "0|~~~",	        
	    ],
	    vec![
	        "  000",
	        "0|<> ",
	        "0|~~~",
	    ]);   
	}

	#[test]
	fn it_creates_horizontal_middle_between_ends() {
		do_test(vec![
	        "  000",
	        "0|~~~",
	        "0|<*>",
	        "0|~~~",
	    ],
	    vec![
	        "  000",	    
	        "0|~~~",
	        "0|<->",
	        "0|~~~",
	    ]);   
	}	

	#[test]
	fn it_creates_horizontal_middle_between_ends_on_border() {
		do_test(vec![
	        "  000",
	        "0|<*>",
	        "0|~~~",
	    ],
	    vec![
	        "  000",	    
	        "0|<->",
	        "0|~~~",
	    ]);   
	}		

	#[test]
	fn it_doesnt_create_horizontal_middle_on_border_without_ends() {
		do_test(vec![
	    	// Don't convert this ship. We can't tell if it will be an end,
	    	// a middle, or a dot
	        "  000",
	        "0| * ", 
	        "0|~~~",
	    ],
	    vec![
	        "  000",
	        "0| * ", 
	        "0|~~~",
	    ]);   
	}	
}