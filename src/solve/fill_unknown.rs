/////////////////////////////////////////////////////////////////////
//
// Solutions that fill unknown squares

use crate::board::*;
use crate::error::*;
use crate::square::*;


pub fn fill_with_water(board: &mut Board) -> Result<()>  {
    let layout = board.layout;
    for row_or_col in layout.rows_and_cols() {
        if board.ship_squares_remaining(row_or_col) == 0 {
            board.replace_unknown(row_or_col, Square::Water)?
        }
    }
    Ok(())
}

// If number of Unknown squares on an axis == number of ships unaccounted for,
// fill the blank spots with ships
pub fn fill_with_ships(board: &mut Board) -> Result<()> {
    let layout = board.layout;
    for row_or_col in layout.rows_and_cols() {
        let num_unknown = row_or_col.coords()
            .filter(|coord| board[*coord] == Square::Unknown)
            .count();

        if num_unknown == board.ship_squares_remaining(row_or_col) {
            board.replace_unknown(row_or_col, Square::ShipSquare(ShipSquare::Any))?
        }
    }

    Ok(())
}

#[test]
fn it_fills_with_ships() -> Result<()> {
    let mut board = Board::new(&vec![
        "  0110",
        "2|    ", // don't change, unknown != ships remaining
        "2|~  ~", // do change, unknown == ships remaining
        "0|~~~~", // don't barf
    ])?;

    let _ = fill_with_ships(&mut board)?;

    let expected = vec![
        "  0000".to_string(),
        "2|    ".to_string(),
        "0|~**~".to_string(),
        "0|~~~~".to_string(),
    ];
    assert_eq!(board.to_strings(), expected);

    Ok(())
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn it_fills_with_water() -> Result<()> {
	    let mut board = Board::new(&vec![
	        "  0011",
	        "0|~*  ",
	        "2|~*  ",
	    ])?;

	    let _ = fill_with_water(&mut board)?;

	    let result = board.to_strings();
	    let expected = vec![
	        "  0011".to_string(),
	        "0|~*~~".to_string(),
	        "2|~*  ".to_string(),       
	    ];

	    assert_eq!(result, expected);

        Ok(())
	}	
}
