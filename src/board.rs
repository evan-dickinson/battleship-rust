#![allow(clippy::needless_return)]

use std::ops::Index;
use std::collections::HashMap;

use crate::square::*;
use crate::layout::*;
use crate::parse::*;

pub struct Board {
    ships_to_find: HashMap<usize, usize>, // ship size => count of ships remaining
    squares: Vec<Vec<Square>>,
    ships_remaining_for_col: Vec<usize>,
    ships_remaining_for_row: Vec<usize>,
    pub layout : Layout,
}

impl Board {
    /////////////////////////////////////////////////////////////////////
    //
    // Board creation

    pub fn new(text_lines : Vec<&str>) -> Self {
        let mut text = text_lines.join("\n");
        text.push_str("\n.");

        return parse_board(&text);
    }

    pub fn new_from_data(squares: Vec<Vec<Square>>, 
        ships_remaining_for_row: Vec<usize>,
        ships_remaining_for_col: Vec<usize>,
        ships_to_find: HashMap<usize, usize>) -> Self {

        let layout = Layout {
            num_rows: squares.len(),
            num_cols: squares[0].len(),            
        };

        return Board {
            squares,
            ships_remaining_for_col,
            ships_remaining_for_row,
            layout,
            ships_to_find
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
                let mut row_text = format!("{}|", row_count);
                let squares = row.iter().map(Square::to_string);
                row_text.extend(squares);

                row_text
            })
            .collect();
    }

    fn format_ships_to_find(&self) -> Option<String> {
        if self.ships_to_find.is_empty() {
            return None;
        }

        let mut out = "ships: ".to_string();

        let mut ship_sizes = self.ships_to_find.keys().cloned().collect::<Vec<_>>();
        ship_sizes.sort();
        ship_sizes.reverse();

        let ship_strings = ship_sizes.iter()
            .map(|&ship_size| {
                let count = self.ships_to_find_for_size(ship_size);
                let msg = format!("{}sq x {}", ship_size, count);
                msg.to_string()
            })
            .collect::<Vec<_>>();

        let ship_string = ship_strings.join(", ");

        out.push_str(&ship_string);
        out.push('.');
        return Some(out);
    }

    pub fn to_strings(&self) -> Vec<String> {
        let mut out = Vec::new();

        if let Some(ships_row) = self.format_ships_to_find() {
            out.push(ships_row);
        }

        let header_row = self.format_col_headers();
        out.push(header_row);

        let mut other_rows = self.format_rows();
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
        else if curr_value == Square::Ship(Ship::AnyMiddle) {
            let is_middle = match value {
                Square::Ship(Ship::AnyMiddle) |
                Square::Ship(Ship::VerticalMiddle) |
                Square::Ship(Ship::HorizontalMiddle) => true,
                _ => false,
            };
            assert_eq!(is_middle, true);

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

    // Count number of ships remaining in the given row/col
    pub fn ships_remaining(&self, row_or_col : RowOrCol) -> usize {
        return match row_or_col.axis {
            Axis::Row => self.ships_remaining_for_row[row_or_col.index],
            Axis::Col => self.ships_remaining_for_col[row_or_col.index],
        }
    }

    // Enumerate all the sizes of ships that remain to be found
    pub fn remaining_ship_sizes<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        return self.ships_to_find.iter()
            .filter_map(|(&ship_size, &count)|
                if count > 0 {
                    Some(ship_size)
                }
                else {
                    None
                }
            );
    }

    // How many ships of a given size remain to be found
    pub fn ships_to_find_for_size(&self, ship_size: usize) -> usize {
        if let Some(&total) = self.ships_to_find.get(&ship_size) {
            let found = self.count_found_ships(ship_size);

            return total - found;
        }
        else {
            return 0;
        }
    }

    fn count_found_ships(&self, ship_size: usize) -> usize {
        let mut ship_count = 0;

        self.iterate_possible_ships(ship_size, |coord, incrementing_axis| {
            if self.ship_exists_at_coord(ship_size, coord, incrementing_axis) {
                ship_count += 1;
            }            
        });

        return ship_count;
    }

    fn ship_exists_at_coord(&self, ship_size: usize, coord: Coord, incrementing_axis: Axis) -> bool {
        return self.test_ship_at_coord(ship_size, coord, incrementing_axis,
            |coord, square_idx| {
                let expected = Ship::expected_square_for_ship(ship_size, square_idx, incrementing_axis);
                self[coord] == Square::Ship(expected)
            });
    }

    // TODO: This should return an interator, but I'm not sure how to do that
    pub fn iterate_possible_ships<F>(&self, ship_size: usize, mut callback : F)
    where F: FnMut(Coord, Axis) {
        // When placing size = 1, we don't increment the coordinate so axis doesn't matter. But if we
        // search by both axes, every coord will match twice. So only search by one axis, and we only match
        // every candidate coordinate once.
        let axes = if ship_size == 1 { 
            vec![Axis::Row]
        }
        else {
            vec![Axis::Row, Axis::Col]
        };

        let layout = self.layout;
        for coord in layout.all_coordinates() {
            for incrementing_axis in axes.iter() {
                callback(coord, *incrementing_axis);
            }
        }   
    }

    // Calls a test function repeatedly, for every square in the ship (originating at coord).
    // Returns true if the test function returns true for every coordinate, and if the ship is in bounds.
    pub fn test_ship_at_coord<T>(&self, ship_size: usize, coord: Coord, incrementing_axis: Axis, mut test : T) -> bool
    where T: FnMut(Coord, usize) ->bool {

        for square_idx in 0..ship_size {
            if let Some(coord) = self.layout.offset(coord, square_idx, incrementing_axis) {

                if !test(coord, square_idx) {
                    return false;
                }
            }
            else {
                // Out of bounds
                return false;
            }
        }

        return true; // all tests passed
    }
}

impl Index<Coord> for Board {
    type Output = Square;

    fn index(&self, index : Coord) -> &Square {
        return &self.squares[index.row_num][index.col_num];
    }
}

#[cfg(test)] use crate::test_utils::*;

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
            "0| ~ ",
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

#[cfg(test)]
mod test_find_ships {
    use super::*;

    #[test]
    fn it_finds_ship() {
        let board = Board::new(vec![
            "  000",
            "0|  ^",
            "0| ~|",
            "0|• |",
            "0|  v",
            "0|   ",
            "0|<> ",
        ]);    

        // Find vertical ship
        let coord = Coord { row_num: 0, col_num: 2 };
        let found_ship = board.ship_exists_at_coord(
            /* size */ 4, coord, Axis::Row
            );
        assert_eq!(found_ship, true);

        // Find horizontal ship
        let coord = Coord { row_num: 5, col_num: 0 };
        let found_ship = board.ship_exists_at_coord(
            /* size */ 2, coord, Axis::Col
            );
        assert_eq!(found_ship, true);        

        // Finds dot
        let coord = Coord { row_num: 2, col_num: 0 };
        let found_ship = board.ship_exists_at_coord(
            /* size */ 1, coord, Axis::Row
            );
        assert_eq!(found_ship, true);        

        // Does not match when size < ship size
        let coord = Coord { row_num: 0, col_num: 2 };
        let found_ship = board.ship_exists_at_coord(
            /* size */ 3, coord, Axis::Row
            );
        assert_eq!(found_ship, false);

        // Does not match when size > ship size
        let coord = Coord { row_num: 0, col_num: 2 };
        let found_ship = board.ship_exists_at_coord(
            /* size */ 5, coord, Axis::Row
            );
        assert_eq!(found_ship, false);                
    }

    #[test]
    fn it_counts_ships() {
        let board = Board::new(vec![
            "  00000",
            "0|  ^ ^",
            "0| ~| |",
            "0|• | |",
            "0|  v v",
            "0|     ",
        ]);

        let count = board.count_found_ships(1);
        assert_eq!(count, 1);

        let count = board.count_found_ships(3);
        assert_eq!(count, 0);    
        
        let count = board.count_found_ships(4);
        assert_eq!(count, 2);      

        let count = board.count_found_ships(5);
        assert_eq!(count, 0);              
    }
}

