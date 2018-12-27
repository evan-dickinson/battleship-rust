/////////////////////////////////////////////////////////////////////
//
// Solutions that fill unknown squares

use crate::square::*;
use crate::board::*;

pub fn fill_with_water(board: &mut Board, changed : &mut bool) {
    for row_or_col in board.layout.rows_and_cols() {
        if board.ships_remaining(row_or_col) == 0 {
            board.replace_unknown(row_or_col, Square::Water, changed);
        }
    }
}

// If number of Unknown squares on an axis == number of ships unaccounted for,
// fill the blank spots with ships
pub fn fill_with_ships(board: &mut Board, changed: &mut bool) {
    for row_or_col in board.layout.rows_and_cols() {
        let num_unknown = board.layout.coordinates(row_or_col)
            .filter(|coord| { board[*coord] == Square::Unknown } )
            .count();

        if num_unknown == board.ships_remaining(row_or_col) {
            board.replace_unknown(row_or_col, Square::Ship(Ship::Any), changed);
        }
    }
}

#[test]
fn it_fills_with_ships() {
    let mut board = Board::new(&vec![
        "  0110",
        "2|    ", // don't change, unknown != ships remaining
        "2|~  ~", // do change, unknown == ships remaining
        "0|~~~~", // don't barf
    ]);

    let mut _changed = false;
    fill_with_ships(&mut board, &mut _changed);

    let expected = vec![
        "  0000".to_string(),
        "2|    ".to_string(),
        "0|~**~".to_string(),
        "0|~~~~".to_string(),
    ];
    assert_eq!(board.to_strings(), expected);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn it_fills_with_water() {
	    let mut board = Board::new(&vec![
	        "  0011",
	        "0|~*  ",
	        "2|~*  ",
	    ]);

	    let mut _changed = false;
	    fill_with_water(&mut board, &mut _changed);

	    let result = board.to_strings();
	    let expected = vec![
	        "  0011".to_string(),
	        "0|~*~~".to_string(),
	        "2|~*  ".to_string(),       
	    ];

	    assert_eq!(result, expected);
	}	
}
