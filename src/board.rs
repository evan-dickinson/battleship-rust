use std::ops::Index;

use square::*;
use layout::*;

pub struct Board {
    squares: Vec<Vec<Square>>,
    ships_remaining_for_col: Vec<usize>,
    ships_remaining_for_row: Vec<usize>,
    pub layout : Layout,
}

impl Board {
    /////////////////////////////////////////////////////////////////////
    //
    // Board creation

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
        //  "1|~~* ",
        //  "1|  *~",
        // ];

    pub fn new(board_text : Vec<&str>) -> Self {
        let first_line = board_text[0];
        let other_lines = &board_text[1..board_text.len()];

        // TODO: Should validate sizes of ships remaining

        let squares = Board::parse_squares(other_lines); 
        let layout = Layout {
            num_rows: squares.len(),
            num_cols: squares[0].len(),            
        };


        return Board {
            squares: squares,
            ships_remaining_for_col: 
                Board::parse_ships_remaining_for_col(first_line),
            ships_remaining_for_row:            
                Board::parse_ships_remaining_for_row(other_lines),
            layout: layout,
        };
    }

    /////////////////////////////////////////////////////////////////////
    //
    // Printing / converting to string

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

    pub fn print(&self) {
        for str in self.to_strings() {
            println!("{}", str);
        }        
    }

    /////////////////////////////////////////////////////////////////////
    //
    // Setting values

    // changed: set to true if board[index] != value, othewise do not set
    pub fn set(&mut self, index : Coord, value : Square, changed : &mut bool) {
        let curr_value = self.squares[index.row_num][index.col_num];

        if curr_value == value {
            return;
        }

        // Check for logic errors. You can only:
        // - Refine Ship::Any to a more specific kind of ship
        // - Change Unknown to another value

        let was_already_ship;
        if curr_value == Square::Ship(Ship::Any) {
            let is_ship = match value {
                Square::Ship(_) => true,
                _               => false,
            };
            assert_eq!(is_ship, true);

            was_already_ship = true;
        }
        else {
            assert_eq!(curr_value, Square::Unknown);

            was_already_ship = false;
        }

        self.squares[index.row_num][index.col_num] = value;

        // Update ships remaining
        if let Square::Ship(_) = value {
            if !was_already_ship {
                self.ships_remaining_for_row[index.row_num] -= 1;
                self.ships_remaining_for_col[index.col_num] -= 1;
            }
        }

        *changed = true;
    }

    pub fn set_bulk(&mut self, indexes : &mut Iterator<Item = Coord>, value : Square, changed : &mut bool) {
        indexes.for_each(|index| {
            self.set(index, value, changed);
        });
    }

    // Count number of ships remaining in the given row/col
    pub fn ships_remaining(&self, row_or_col : RowOrCol) -> usize {
        return match row_or_col.axis {
            Axis::Row => self.ships_remaining_for_row[row_or_col.index],
            Axis::Col => self.ships_remaining_for_col[row_or_col.index],
        }
    }

    // In the given row/col, replace all Unknown squares with the specified value
    pub fn replace_unknown(&mut self, row_or_col : RowOrCol, new_value : Square, changed : &mut bool) {
        for coord in self.layout.coordinates(row_or_col) {
            if self[coord] == Square::Unknown {
                self.set(coord, new_value, changed);
            }
        }
    }

    /////////////////////////////////////////////////////////////////////
    //
    // Contents of the board
    pub fn is_solved(&self) -> bool {
        return self.layout.all_coordinates()
            .all(|coord| self[coord] != Square::Unknown);
    }
}

impl Index<Coord> for Board {
    type Output = Square;

    fn index(&self, index : Coord) -> &Square {
        return &self.squares[index.row_num][index.col_num];
    }
}

#[cfg(test)] use test_utils::*;

#[cfg(test)]
mod test {
    use super::*;

	#[test]
	fn it_sets_changed() {
	    let mut board = Board::new(vec![
	        "  001",
	        "0|   ",
	        "1|~  ",
	    ]);

	    let mut changed = false;

	    // Setting an unchanged square leaves changed alone
	    let coord = Coord { row_num: 1, col_num: 0};
	    board.set(coord, Square::Water, &mut changed);
	    assert_eq!(changed, false);

	    // Setting a changed square sets changed
	    let coord = Coord { row_num: 0, col_num: 0};
	    board.set(coord, Square::Water, &mut changed);
	    assert_eq!(changed, true);

	    // Once changed is true, don't set it back to false
	    let coord = Coord { row_num: 0, col_num: 0};
	    board.set(coord, Square::Water, &mut changed);
	    assert_eq!(changed, true);
	}

    #[test]
    fn it_returns_num_rows() {
        let board = make_test_board();
        assert_eq!(board.layout.num_rows, 2);
    }

    #[test]
    fn it_returns_num_cols() {
        let board = make_test_board();
        assert_eq!(board.layout.num_cols, 4);
    }    

    #[test]
    fn it_accesses_with_index() {
        let mut board = make_test_board();
        let coord1 = Coord {
            row_num: 1,
            col_num: 0,
        };

        assert_eq!(board[coord1], Square::Unknown);
        let mut _changed = false;
        board.set(coord1, Square::Water, &mut _changed);
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
        let mut coords = board.layout.coordinates(RowOrCol {
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

        let mut _changed = false;
        board.set(coord, Square::Ship(Ship::Any), &mut _changed);

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

    #[test]
    fn it_accesses_col_contents() {
        let board = Board::new(vec![
            "  000",
            "0| ^ ",
            "0| | ",
            "0| v ",
            "0  ~ ",
        ]);

        let col = RowOrCol {
            axis:  Axis::Col,
            index: 1,
        };

        let mut col_coords = board.layout.coordinates(col);

        let mut expected_coord;

        expected_coord = Coord{
            row_num: 0,
            col_num: 1,
        };
        assert_eq!(col_coords.next(), Some(expected_coord));
        assert_eq!(board[expected_coord], Square::Ship(Ship::TopEnd));

        expected_coord = Coord{
            row_num: 1,
            col_num: 1,
        };
        assert_eq!(col_coords.next(), Some(expected_coord));
        assert_eq!(board[expected_coord], Square::Ship(Ship::VerticalMiddle));

        expected_coord = Coord{
            row_num: 2,
            col_num: 1,
        };
        assert_eq!(col_coords.next(), Some(expected_coord));
        assert_eq!(board[expected_coord], Square::Ship(Ship::BottomEnd));

        expected_coord = Coord{
            row_num: 3,
            col_num: 1,
        };
        assert_eq!(col_coords.next(), Some(expected_coord));
        assert_eq!(board[expected_coord], Square::Water);

        assert_eq!(col_coords.next(), None);
    }
}
