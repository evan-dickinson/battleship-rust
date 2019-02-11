use crate::board::*;
use crate::error::*;
use crate::layout::*;
use crate::neighbor::*;
use crate::square::*;

pub fn place_ships_next_to_ends(board: &mut Board) -> Result<()> {
	let layout = board.layout;
    for coord in layout.all_coordinates() {
        let neighbor = match board[coord] {
            Square::ShipSquare(ShipSquare::TopEnd)    => Neighbor::S,
            Square::ShipSquare(ShipSquare::BottomEnd) => Neighbor::N,
            Square::ShipSquare(ShipSquare::LeftEnd)   => Neighbor::E,
            Square::ShipSquare(ShipSquare::RightEnd)  => Neighbor::W,
            _                                         => continue,
        };

        // Convert from Option<Coord> to Result<Coord>, so we can return an error
        // if neighbor is out of bounds. That would mean that, for example, the
        // top end of a ship is on the last row of the board. No place to put the
        // rest of the ship.
        let neighbor_coord_result: Result<Coord> = coord.neighbor(neighbor)
        	.ok_or_else(
        		|| format!("Square {:?} at {:?} wants a neighbor to the {:?}, but no place to put it.",
        	 		board[coord], coord, neighbor).into()
        		);
        let neighbor_coord = neighbor_coord_result?;

        if board[neighbor_coord] == Square::Unknown {
        	board.set(neighbor_coord, Square::ShipSquare(ShipSquare::Any))?;
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

	    place_ships_next_to_ends(&mut board)?;
	    assert_eq!(board.to_strings(), expected);        

	    Ok(())
	}

	#[test]
	fn it_places_ships_next_to_ends() -> Result<()> {
	    do_test(vec![
	        "  00100",
	        "0|  ^  ",
	        "1|     ",
	    ],
		vec![
	        "  00000",
	        "0|  ^  ",
	        "0|  *  ",    
	    ])
	}

	#[test]
	fn it_errors_if_no_place_for_a_ship() -> Result<()> {
	    let before = vec![
	        "  00100",
	        "1|  v  ",
	    ];

		let mut board = Board::new(&before)?;
	    let result = place_ships_next_to_ends(&mut board);

	    assert_eq!(result.is_err(), true);

	    Ok(())
	}

	#[test]
	fn it_doesnt_overwrite_existing_ship() -> Result<()> {
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
	   	])
	}
}	
