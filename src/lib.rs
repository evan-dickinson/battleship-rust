// rustc --crate-type lib --emit llvm-ir lib.rs -O

pub mod client;
pub mod network;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Coord {
	x : usize,
	y : usize
}


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
		
		// TODO: Should ensure that all rows have equal length

		return match rows {
			Ok(squares) => Ok(Board {
				squares: squares
			}),
			Err(msg)    => Err(msg)
		}
	}

	pub fn num_rows(&self) -> usize {
		return self.squares.len();
	}

	pub fn num_cols(&self) -> usize {
		return self.squares[0].len();
	}

	pub fn row(&self, row_num : usize) -> impl Iterator<Item = (Coord, &Square)>  {
		return self.squares[row_num].iter().enumerate().map(move |(col_num, square)| {
			let location = Coord {
				x: col_num,
				y: row_num
			};
			return (location, square)
		});
	}

	pub fn col(&self, col_num : usize) -> impl Iterator<Item = (Coord, &Square)> {
		let range = 0..self.num_rows();
		return range.map(move |row_num| {
			let location = Coord {
				x: col_num,
				y: row_num
			};
			let square = &(self.squares[row_num][col_num]);
			return (location, square);
		});
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
    fn it_returns_num_rows() {
    	let board = make_test_board();
    	assert_eq!(board.num_rows(), 2);
    }

    #[test]
    fn it_returns_num_cols() {
    	let board = make_test_board();
    	assert_eq!(board.num_cols(), 4);
    }    

    #[test]
    fn it_gets_a_row() {
    	let board = make_test_board();
    	let row1 : Vec<(Coord, &Square)> = board.row(0).collect();

    	let expected_row = vec![
    		( Coord { x: 0, y: 0 }, Square::Water),
    		( Coord { x: 1, y: 0 }, Square::Water),
    		( Coord { x: 2, y: 0 }, Square::Ship),
    		( Coord { x: 3, y: 0 }, Square::Unknown)
    	];

    	let items_equal = expected_row.iter()
    		.zip(row1.iter())
    		.all(|(a, b)| { 
    			let (a_coord, a_square) = a;
    			let (b_coord, b_square) = b;

    			return a_coord == b_coord && a_square == *b_square;
    		});

    	assert_eq!(row1.len(), expected_row.len());
    	assert!(items_equal);
    }

    #[test]
    fn it_gets_a_col() {
    	let board = make_test_board();
    	let col2 : Vec<(Coord, &Square)> = board.col(2).collect();

    	let expected_col = vec![
    		( Coord { x: 2, y: 0 }, Square::Ship),
    		( Coord { x: 2, y: 1 }, Square::Ship),
    	];

    	let items_equal = expected_col.iter()
    		.zip(col2.iter())
    		.all(|(a, b)| { 
    			let (a_coord, a_square) = a;
    			let (b_coord, b_square) = b;

    			return a_coord == b_coord && a_square == *b_square;
    		});

    	assert_eq!(col2.len(), expected_col.len());
    	assert!(items_equal);
    }

}
