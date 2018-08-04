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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Axis {
	Row,
	Col
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RowOrCol {
	pub axis : Axis,
	pub index : usize
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
	ships_remaining_for_col: Vec<usize>,
	ships_remaining_for_row: Vec<usize>,
}

impl Board {
	fn parse_ships_remaining_for_col(count_line : &str) -> Vec<usize> {
		// skip the first 2 chars. They're blanks.
		return count_line.chars().skip(2).map(|char| {
				char.to_string().parse().unwrap()
			})
			.collect();
	}

	fn parse_ships_remaining_for_row(lines : &[&str]) -> Vec<usize> {
		return lines.iter().map(|line| {
				let c = line.chars().next().unwrap(); // get first char in the string
				return c.to_string().parse().unwrap();
			})
			.collect();
	}

	fn parse_squares(lines : &[&str]) -> Vec<Vec<Square>> {
		// TODO: Should ensure that all rows have equal length
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

		// TODO: Should validate sizes of ships remaining

    	return Board {
    		squares: Board::parse_squares(other_lines),
    		ships_remaining_for_col: 
    			Board::parse_ships_remaining_for_col(first_line),
			ships_remaining_for_row:     		
				Board::parse_ships_remaining_for_row(other_lines),
    	};
    }

    fn format_col_headers(&self) -> String {
    	let prefix = "  ".to_string(); // start the line with two blanks
    	return self.ships_remaining_for_col.iter()
	    	.map(|x| {
	            return x.to_string();
	        })
	        .fold(prefix, |mut acc, x| {
	            acc.push_str(&x);
	            return acc;
	        });
    }

    fn format_rows(&self) -> Vec<String> {
    	return self.squares.iter()
    		.enumerate()
    		.map(|(row_num, row)| {
    			let row_count = self.ships_remaining_for_row[row_num];
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
    	let first_row = self.format_col_headers();
    	let mut other_rows = self.format_rows();

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

	pub fn set(&mut self, index : Coord, value : Square) {
		assert!(self.squares[index.row_num][index.col_num] == Square::Unknown);

		self.squares[index.row_num][index.col_num] = value;

		// Update ships remaining
		if value == Square::Ship {
			self.ships_remaining_for_row[index.row_num] -= 1;
			self.ships_remaining_for_col[index.col_num] -= 1;
		}
	}

	pub fn rows_and_cols(&self) -> impl Iterator<Item = RowOrCol> {
		let rows = (0 .. self.num_rows()).map(|row_num| {
			RowOrCol {
				axis: Axis::Row,
				index: row_num
			}
		});

		let cols = (0 .. self.num_cols()).map(|col_num| {
			RowOrCol {
				axis: Axis::Col,
				index: col_num
			}
		});		

		return rows.chain(cols);
	}

	// Return all the coordinates along the specified row or col
	pub fn coordinates(&self, row_or_col : RowOrCol) -> impl Iterator<Item = Coord>  {
		// Count number of items in the minor axis
		let minor_axis_ubound = match row_or_col.axis {
			Axis::Row => self.num_cols(),
			Axis::Col => self.num_rows()
		};
		let range = 0 .. minor_axis_ubound;

		let major_axis_idx = row_or_col.index;
		return range.map(move |minor_axis_idx| {
			return match row_or_col.axis {
				Axis::Row => Coord {
					row_num: major_axis_idx,
					col_num: minor_axis_idx,
				},
				Axis::Col => Coord {
					row_num: minor_axis_idx,
					col_num: major_axis_idx
				}
			}
		});
	}

	// Count number of ships remaining in the given row/col
	//
	// TODO: Use "ships remaining" as a name more widely
	pub fn ships_remaining(&self, row_or_col : RowOrCol) -> usize {
		return match row_or_col.axis {
			Axis::Row => self.ships_remaining_for_row[row_or_col.index],
			Axis::Col => self.ships_remaining_for_col[row_or_col.index],
		}		
	}

	// In the given row/col, replace all Unknown squares with the specified value
	pub fn replace_unknown(&mut self, row_or_col : RowOrCol, new_value : Square) {
		for coord in self.coordinates(row_or_col) {
			if self[coord] == Square::Unknown {
				self.set(coord, new_value);
			}
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

	fn fill_with_water(board: &mut Board) {
		for row_or_col in board.rows_and_cols() {
			if board.ships_remaining(row_or_col) == 0 {
				board.replace_unknown(row_or_col, Square::Water);
			}
		}
	}


	// If number of Unknown squares on an axis == number of ships unaccounted for,
	// fill the blank spots with ships
	fn fill_with_ships(board: &mut Board) {
		for row_or_col in board.rows_and_cols() {
			// Count unknown squares on this row or col
			let num_unknown = board.coordinates(row_or_col)
				.filter(|coord| { board[*coord] == Square::Unknown } )
				.count();				

			if num_unknown == board.ships_remaining(row_or_col) {
				board.replace_unknown(row_or_col, Square::Ship);
			}
		}
	}

	#[test]
	fn it_fills_with_water() {
    	let mut board = Board::new(vec![
    	    "  0011",
    		"0|~*  ",
    		"2|~*  ",
    	]);

    	fill_with_water(&mut board);

    	let result = board.to_strings();
    	let expected = vec![
    	    "  0011".to_string(),
    		"0|~*~~".to_string(),
    		"2|~*  ".to_string(),    	
    	];

    	assert_eq!(result, expected);
	}

	#[test]
	fn it_fills_with_ships() {
		let mut board = Board::new(vec![
    	    "  0011",
    		"0|~*~~",
    		"2|~*  ",
		]);

		fill_with_ships(&mut board);

		let expected = vec![
    	    "  0000".to_string(),
    		"0|~*~~".to_string(),
    		"0|~***".to_string(),    			
		];
		assert_eq!(board.to_strings(), expected);
	}
}


#[cfg(test)]
mod board_tests {
	use super::*;

    pub fn make_test_board() -> Board {
    	let text = vec![
    	    "  1101",
    		"1|~~* ",
    		"2|  *~",
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
    fn it_accesses_with_index() {
    	let mut board = make_test_board();
    	let coord1 = Coord {
    		row_num: 1,
    		col_num: 0,
    	};

    	assert_eq!(board[coord1], Square::Unknown);
    	board.set(coord1, Square::Water);
    	assert_eq!(board[coord1], Square::Water);

    	let coord2 = Coord {
    		row_num: 1,
    		col_num: 3,
    	};

    	assert_eq!(board[coord2], Square::Water);
    }

    #[test]
    fn it_accesses_col() {
    	let board = make_test_board();
    	let mut coords = board.coordinates(RowOrCol {
    		axis:  Axis::Col,
    		index: 1
    	});

    	assert_eq!(coords.next(), Some(Coord {
    		row_num: 0,
    		col_num: 1,
    	}));
    	assert_eq!(coords.next(), Some(Coord {
    		row_num: 1,
    		col_num: 1,
    	}));
    	assert_eq!(coords.next(), None);
    }

    #[test]
    fn it_counts_ships_remaining() {
    	let board = make_test_board();

    	assert_eq!(board.ships_remaining(
    		RowOrCol { 
    			axis:  Axis::Row,
    			index: 0
    		}), 1);
    	assert_eq!(board.ships_remaining(
    		RowOrCol { 
    			axis:  Axis::Row,
    			index: 1
    		}), 2);
    	assert_eq!(board.ships_remaining(
    		RowOrCol { 
    			axis:  Axis::Col,
    			index: 0
    		}), 1);
    	assert_eq!(board.ships_remaining(
    		RowOrCol { 
    			axis:  Axis::Col,
    			index: 2
    		}), 0);    	
    }

    #[test]
    fn it_adjusts_ships_remaining_after_set() {
    	let mut board = make_test_board();
    	let coord = Coord {
    		row_num: 1,
    		col_num: 0,
    	};

    	assert_eq!(board.ships_remaining(
    		RowOrCol { 
    			axis:  Axis::Row,
    			index: coord.row_num
    		}), 2);
    	assert_eq!(board.ships_remaining(
    		RowOrCol { 
    			axis:  Axis::Col,
    			index: coord.col_num
    		}), 1);

    	board.set(coord, Square::Ship);

    	// ships remaining has decreased
    	assert_eq!(board.ships_remaining(
    		RowOrCol { 
    			axis:  Axis::Row,
    			index: coord.row_num
    		}), 2 - 1);
    	assert_eq!(board.ships_remaining(
    		RowOrCol { 
    			axis:  Axis::Col,
    			index: coord.col_num
    		}), 1 - 1);    	

    }
}



