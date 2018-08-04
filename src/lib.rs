// rustc --crate-type lib --emit llvm-ir lib.rs -O

use std::ops::Index;
use std::fmt;

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

impl fmt::Display for Square {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let char = match self {
			Square::Unknown => ' ',
			Square::Ship    => '*',
			Square::Water   => '~',
		};

        return write!(f, "{}", char)
    }
}

impl From<char> for Square {
	fn from(square_char : char) -> Self {
		return match square_char {
			' ' => Square::Unknown,
			'*' => Square::Ship,
			'~' => Square::Water,
			_   => panic!("Unknown char".to_string())
		}		
	}
}

pub struct Board {
	squares: Vec<Vec<Square>>,
	col_counts: Vec<usize>,
	row_counts: Vec<usize>,
}

impl Board {
	fn parse_col_counts(count_line : &str) -> Vec<usize> {
		// skip the first 2 chars. They're blanks.
		return count_line.chars().skip(2).map(|char| {
				char.to_string().parse().unwrap()
			})
			.collect();
	}

	fn parse_row_counts(lines : &[&str]) -> Vec<usize> {
		return lines.iter().map(|line| {
				let c = line.chars().next().unwrap(); // get first char in the string
				return c.to_string().parse().unwrap();
			})
			.collect();
	}

	fn parse_squares(lines : &[&str]) -> Vec<Vec<Square>> {
		return lines.iter().map(|line| {
				return line.chars()
					.skip(2)
					.map(Square::from)
					.collect();
			})
			.collect();
	}

    	// let text = vec![
    	//  "  1001"
    	// 	"1|~~* ",
    	// 	"1|  *~",
    	// ];

    pub fn new(board_text : Vec<&str>) -> Self {
    	let first_line = board_text[0];
    	let other_lines = &board_text[1..board_text.len()];

		// TODO: Should ensure that all rows have equal length
		// TODO: Should validate row_counts and col_counts

    	let col_counts = Board::parse_col_counts(first_line);
    	let row_counts = Board::parse_row_counts(other_lines);
    	let squares    = Board::parse_squares(other_lines);

    	return Board {
    		squares: squares,
    		col_counts: col_counts,
    		row_counts: row_counts,
    	};
    }

    fn make_col_counts(&self) -> String {
    	let prefix = "  ".to_string(); // start the line with two blanks
    	return self.col_counts.iter()
	    	.map(|x| {
	            return x.to_string();
	        })
	        .fold(prefix, |mut acc, x| {
	            acc.push_str(&x);
	            return acc;
	        });
    }

    fn make_rows(&self) -> Vec<String> {
    	return self.squares.iter()
    		.enumerate()
    		.map(|(row_num, row)| {
    			let row_count = self.count_for_row(row_num);
    			let row_head = format!("{}|", row_count);

    			return row.iter()
    				.map(Square::to_string)
    				.fold(row_head, |mut acc, square_str| {
    					acc.push_str(&square_str);
    					return acc;
    				})
    		})
    		.collect();
    }

    pub fn to_strings(&self) -> Vec<String> {
    	let first_row = self.make_col_counts();
    	let mut other_rows = self.make_rows();

    	let mut out = Vec::new();
    	out.push(first_row);
     	out.append(&mut other_rows);

    	return out;
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

	#[test]
	fn it_fills_row_with_water() {
    	let mut board = Board::new(vec![
    	    "  0000",
    		"0|~*  ",
    		"1|~*  ",
    	]);

    	fill_empty_rows_with_water(&mut board);

    	let result = board.to_strings();
    	let expected = vec![
    	    "  0000".to_string(),
    		"0|~*~~".to_string(),
    		"1|~*  ".to_string(),    	
    	];

    	assert_eq!(result, expected);
	}
}


#[cfg(test)]
mod board_tests {
	use super::*;

    pub fn make_test_board() -> Board {
    	let text = vec![
    	    "  1001",
    		"1|~~* ",
    		"1|  *~",
    	];

		return Board::new(text);
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

