use crate::board::*;
use crate::error::*;
use crate::neighbor::*;
use crate::square::*;

use smallvec::SmallVec;

// Convert an AnyMiddle to a specific type of middle, based on
// whether or not it's surrounded by water.
pub fn specify_middle(board: &mut Board) -> Result<()> {
    let layout = board.layout;
    let coords = layout.all_coordinates()
        .filter(|&coord| Square::ShipSquare(ShipSquare::AnyMiddle) == board[coord])
        .collect::<SmallVec<[_; 32]>>();

    for coord in coords {
    	let is_surrounded_vert = [Neighbor::N, Neighbor::S].iter()
    		.filter_map(|&neighbor| coord.neighbor(neighbor))
    		.any(|coord| board[coord] == Square::Water);

    	let is_surrounded_horz = [Neighbor::E, Neighbor::W].iter()
    		.filter_map(|&neighbor| coord.neighbor(neighbor))
    		.any(|coord| board[coord] == Square::Water);

    	ensure!(!(is_surrounded_horz && is_surrounded_vert), 
    		"Square at {:?} is an AnyMiddle, but it has neighbors to the north/south and east/west",
    		coord);

  		// If we're surrounded vertically then this ship must be laid out horizontally,
  		// and vice versa.
    	if is_surrounded_vert {
    		board.set(coord, Square::ShipSquare(ShipSquare::HorizontalMiddle))?;
    	}
    	else if is_surrounded_horz {
			board.set(coord, Square::ShipSquare(ShipSquare::VerticalMiddle))?;
    	}
    }

    Ok(())
}

#[cfg(test)]
mod test {
	use super::*;

	fn do_test(before: Vec<&str>, after: Vec<&str>) -> Result<()> {
		let mut board = Board::new(&before)?;
		let expected = after.iter().map(|x| x.to_string()).collect::<Vec<_>>();

	    specify_middle(&mut board)?;
	    assert_eq!(board.to_strings(), expected);      

	    Ok(())  
	}

	#[test]
	fn it_specifies_vertical_middle_surrounded_by_water() -> Result<()> {
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
	    ])
	}

	#[test]
	fn it_specifies_vertical_middle_with_water_on_one_side() -> Result<()> {
	    do_test(vec![
	        "  00200",
	        "1|     ",
	        "0| ~☐  ",
	        "1|     ",
	    ],
		vec![
	        "  00200",
	        "1|     ",
	        "0| ~|  ",
	        "1|     ", 
	    ])
	}	

	#[test]
	fn it_specifies_vertical_middle_at_edge_of_board() -> Result<()> {
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
	    ])
	}	

	#[test]
	fn it_specifies_horizontal_middle_surrounded_by_water() -> Result<()> {
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
	    ])
	}	

	#[test]
	fn it_specifies_horizontal_middle_at_edge_of_board() -> Result<()> {
	    do_test(vec![
	        "  01010",
	        "0|  ~  ",
	        "2|  ☐  ",
	    ],
		vec![
	        "  01010",
	        "0|  ~  ",
	        "2|  -  ",
	    ])
	}
}