// rustc --crate-type lib --emit llvm-ir lib.rs -O

use std::ops::Index;

pub mod client;
pub mod network;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Coord {
	row_num : usize,
	col_num : usize,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Square {
	Unknown,
	Water,
	Ship
}

pub struct Board {
	squares: Vec<Vec<Square>>,
	col_counts: Vec<usize>,
	row_counts: Vec<usize>,
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

	pub fn new(board_text : Vec<&str>,  row_counts : Vec<usize>, col_counts : Vec<usize>) -> Result<Self, String> {
		let foo = board_text.iter().map( |row_str| {
			Board::str_to_row(row_str)
		});

		let rows : Result<Vec<Vec<Square>>, String> = foo.collect();

		// TODO: Should ensure that all rows have equal length
		// TODO: Should validate row_counts and col_counts
		// TODO: How to implement this with the ? operator

		return match rows {
			Ok(squares) => Ok(Board {
				squares: squares,
				col_counts: col_counts,
				row_counts: row_counts
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

	pub fn count_for_row(&self, row_num : usize) -> usize {
		return self.row_counts[row_num];
	}

	pub fn count_for_col(&self, col_num : usize) -> usize {
		return self.col_counts[col_num];
	}	

	pub fn row(&self, row_num : usize) -> impl Iterator<Item = Coord> {
		let range = 0..self.num_cols();
		return range.map(move |col_num| {
			return Coord {
				col_num: col_num,
				row_num: row_num
			};
		});
	}

	pub fn col(&self, col_num : usize) -> impl Iterator<Item = Coord> {
		let range = 0..self.num_rows();
		return range.map(move |row_num| {
			return Coord {
				col_num: col_num,
				row_num: row_num
			};
		});
	}

	pub fn set(&mut self, index : Coord, value : Square) {
		assert!(self.squares[index.row_num][index.col_num] == Square::Unknown);

		self.squares[index.row_num][index.col_num] = value;

		// Update row & col counts
		if value == Square::Ship {
			self.row_counts[index.row_num] -= 1;
			self.col_counts[index.col_num] -= 1;
		}
	}
}

impl Index<Coord> for Board {
	type Output = Square;

	fn index(&self, index : Coord) -> &Square {
		return &self.squares[index.row_num][index.col_num];
	}
}

mod solve {
	use super::*;

	fn fill_empty_rows_with_water(board : &mut Board) {
		for row_num in 0..board.num_rows() {
			if board.count_for_row(row_num) == 0 {
				for coord in board.row(row_num) {
					if board[coord] == Square::Unknown {
						board.set(coord, Square::Water);
					}
				}
			}
		}
	}	
}


#[cfg(test)]
mod board_tests {
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

    pub fn make_test_board() -> Board {
    	let text = vec![
    		"~~* ",
    		"  *~",
    	];
    	let col_counts = vec![
    	    1, 0, 0, 1
    	];
    	let row_counts = vec![
    		1, 1
    	];

		return Board::new(text, col_counts, row_counts).unwrap();
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
    fn it_gets_a_col() {
    	let board = make_test_board();
    	let col2 : Vec<Coord> = board.col(2).collect();

    	let expected_col = vec![
    		Coord { col_num: 2, row_num: 0 },
    		Coord { col_num: 2, row_num: 1 },
    	];

    	assert_eq!(col2.len(), expected_col.len());
    	assert_eq!(col2, expected_col);
    }

    #[test]
    fn it_accesses_with_index() {
    	let mut board = make_test_board();
    	let coord = Coord {
    		row_num: 1,
    		col_num: 0,
    	};

    	assert_eq!(board[coord], Square::Unknown);

    	board.set(coord, Square::Water);

    	assert_eq!(board[coord], Square::Water);
    }
}


#[cfg(bogus)]
mod bogus {
	fn set_row_to_water(&mut board : Board, row_num : usize) {
		for coord in board.row(row_num) {
			*board[coord] = Square::Water;
		}
	}

	fn set_diagonal_neighbors(&mut board : Board, coord : Coord) {
		for neighbor in board.neighbors(coord, NeighborType::Diagonal) {
			*board[neighbor] = Square::Water;
		}
	}
}

