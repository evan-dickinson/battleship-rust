use std::ops::Index;

use neighbor::*;
use square::*;
use layout::*;

pub struct Board {
    squares: Vec<Vec<Square>>,
    ships_remaining_for_col: Vec<usize>,
    ships_remaining_for_row: Vec<usize>,
    pub layout : Layout,
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

    // changed: set to true if board[index] != value, othewise do not set
    pub fn set(&mut self, index : Coord, value : Square, changed : &mut bool) {
        let curr_value = self.squares[index.row_num][index.col_num];

        if curr_value == value {
            return;
        }

        assert_eq!(curr_value, Square::Unknown);

        self.squares[index.row_num][index.col_num] = value;

        // Update ships remaining
        if let Square::Ship(_) = value {
            self.ships_remaining_for_row[index.row_num] -= 1;
            self.ships_remaining_for_col[index.col_num] -= 1;
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
}

impl Index<Coord> for Board {
    type Output = Square;

    fn index(&self, index : Coord) -> &Square {
        return &self.squares[index.row_num][index.col_num];
    }
}


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
}
