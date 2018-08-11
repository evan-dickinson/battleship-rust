/////////////////////////////////////////////////////////////////////
//
// Solutions that convert "any" ships to specific ships

use square::*;
use board::*;
use neighbor::*;
use layout::*;

// - Convert "any" to specific ships:
//   - to dot, when fully surrounded
//   - to end, when surrounded by water and/or edge of board
//   - to vert middle, when surrounded by water on left/right
//   - to horz middle, when surrounded by water on top/bottom
//   - to generic middle, when surrounded by diagonals
//     + check for edge of board, too, not just surrounded by water


fn is_surrounded<'a>(board: &'a Board, index: Coord, neighbors: impl IntoIterator<Item = &'a Neighbor> + 'a) -> bool {
    let mut neighbor_coords = board.layout.coords_for_neighbors(index, neighbors);

    // coords_for_neighbors filterd out coords that are out of bounds. 
    // Now check to see that all remaining neighbors are water.

    return neighbor_coords.all(|coord| board[coord] == Square::Water);
}

pub fn specify_dot(board: &mut Board, changed: &mut bool) {
    let layout = board.layout;
    let ship_coords = {
        layout.all_coordinates()
            .filter(|coord| { board[*coord] == Square::Ship(Ship::Any) })
            .collect::<Vec<_>>()
    };

    for coord in ship_coords {
    	let neighbors = Neighbor::all_neighbors();
    	if is_surrounded(board, coord, neighbors.iter()) {
    		board.set(coord, Square::Ship(Ship::Dot), changed);
    	}
    }
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn it_specifies_dot_surrounded_by_water() {
	    let mut board = Board::new(vec![
	        "  00000",
	        "0| ~~~ ",
	        "0| ~*~ ",
	        "0| ~~~ ",
	    ]);

	    let mut _changed = false;
	    specify_dot(&mut board, &mut _changed);
	    let expected = vec![
	        "  00000",
	        "0| ~~~ ",
	        "0| ~•~ ",
	        "0| ~~~ ",
	    ].iter().map(|x| x.to_string()).collect::<Vec<_>>();
	    assert_eq!(board.to_strings(), expected);        
	}

	#[test]
	fn it_specifies_dot_in_corner() {
	    let mut board = Board::new(vec![
	        "  00000",
	        "0| ~~",
	        "0| ~*",
	    ]);

	    let mut _changed = false;
	    specify_dot(&mut board, &mut _changed);
	    let expected = vec![
	        "  00000",
	        "0| ~~",
	        "0| ~•",
	    ].iter().map(|x| x.to_string()).collect::<Vec<_>>();
	    assert_eq!(board.to_strings(), expected);        
	}	
}