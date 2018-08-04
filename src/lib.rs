// rustc --crate-type lib --emit llvm-ir lib.rs -O

pub mod client;
pub mod network;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Square {
	Unknown,
	Water,
	Ship
}

pub struct Board {
	squares: Vec<Vec<Square>>
}

impl Board {
	fn char_to_square(square_char : char) -> Result<Square, String> {
		match square_char {
			' ' => Ok(Square::Unknown),
			'*' => Ok(Square::Ship),
			'~' => Ok(Square::Water),
			_   => Err("Unknown char".to_string())
		}
	}

	fn str_to_row(line : &str) -> Result<Vec<Square>, String> {
		let row : Result<Vec<Square>, String> = line.chars()
			.map(Board::char_to_square)
			.collect();

		return row;	
	}

	pub fn from_string(text : &str) -> Result<Board, String> {
		let rows : Result<Vec<Vec<Square>>, String> = text.split("\n")
			.map(Board::str_to_row)
			.collect();
		
		return match rows {
			Ok(squares) => Ok(Board {
				squares: squares
			}),
			Err(msg)    => Err(msg)
		}
	}	

	pub fn row(&self, rowNum : usize) -> impl Iterator<Item = &Square>  {
		return self.squares[rowNum].iter();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn it_finds_a_ship() {
    	let result = Board::char_to_square('*');

        assert_eq!(result, Ok(Square::Ship));
    }

    #[test]
    fn it_fails() {
    	let result = Board::char_to_square('q');

        assert_eq!(result, Err("Unknown char".to_string()));
    }

    #[test]
    fn it_makes_a_row() {
    	let line = "~~* ";
    	let row = Board::str_to_row(&line);

    	let expected_row = Ok(vec![
    		Square::Water,
    		Square::Water,
    		Square::Ship,
    		Square::Unknown
    	]);

    	assert_eq!(row, expected_row);
    }

    #[test]
    fn it_fails_to_make_a_row() {
    	let line = "~q~";
    	let row = Board::str_to_row(&line);

    	let expected_row = Err("Unknown char".to_string());

    	assert_eq!(row, expected_row);
    }    

    fn make_test_board() -> Board {
		let text = "~~* \n  *~";

		return Board::from_string(text).unwrap();
    }

    #[test]
    fn it_gets_a_row() {
    	let board = make_test_board();
    	let row1 : Vec<&Square> = board.row(0).collect();

    	let expected_row = vec![
    		Square::Water,
    		Square::Water,
    		Square::Ship,
    		Square::Unknown
    	];

    	let items_equal = expected_row.iter()
    		.zip(row1.iter())
    		.all(|(a, b)| { 
    			*a == **b 
    		});

    	assert_eq!(row1.len(), expected_row.len());
    	assert!(items_equal);
    }
}
