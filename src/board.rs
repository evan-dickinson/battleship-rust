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
    pub layout: Layout,
}

impl Board {
    /////////////////////////////////////////////////////////////////////
    //
    // Board creation

    pub fn new(text_lines : &[&str]) -> Self {        
        let mut text = text_lines.join("\n");
        text.push_str("\n.");

        parse_board(&text)
    }

    pub fn new_from_data(squares: Vec<Vec<Square>>, 
        ships_remaining_for_row: Vec<usize>,
        ships_remaining_for_col: Vec<usize>,
        ships_to_find: HashMap<usize, usize>) -> Self {

        let layout = Layout {
            num_rows: squares.len(),
            num_cols: squares[0].len(),            
        };

        Board {
            squares,
            ships_remaining_for_col,
            ships_remaining_for_row,
            layout,
            ships_to_find
        }
    }

    /////////////////////////////////////////////////////////////////////
    //
    // Printing / converting to string

    fn format_col_headers(&self) -> String {
        let prefix = "  ".to_string(); // start the line with two blanks
        self.ships_remaining_for_col.iter()
            .map(|x| x.to_string() )
            .fold(prefix, |mut acc, x| {
                acc.push_str(&x);
                acc
            })
    }

    fn format_rows(&self) -> Vec<String> {
        self.squares.iter()
            .enumerate()
            .map(|(row_num, row)| {
                let row_count = self.ships_remaining_for_row[row_num];
                let mut row_text = format!("{}|", row_count);
                let squares = row.iter().map(Square::to_string);
                row_text.extend(squares);

                row_text
            })
            .collect()
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
        
        Some(out)
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

        out
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
    pub fn set(&mut self, index: Coord, new_value: Square, changed: &mut bool) {
        let curr_value = self[index];

        if curr_value == new_value {
            return;
        }

        // Check for logic errors. You can only:
        // - Refine ShipSquare::Any to a more specific kind of ship
        // - Refine ShipSquare::AnyMiddle to a more specific kind of middle
        // - Change Unknown to another value
        if curr_value == Square::ShipSquare(ShipSquare::Any) {
            assert!(new_value.is_ship());
        }
        else if curr_value == Square::ShipSquare(ShipSquare::AnyMiddle) {
            assert!(new_value.is_ship_middle());
        }
        else {
            assert_eq!(curr_value, Square::Unknown);
        }

        self.squares[index.row_num][index.col_num] = new_value;

        // Update ships remaining
        if new_value.is_ship() && !curr_value.is_ship() {
            self.ships_remaining_for_row[index.row_num] -= 1;
            self.ships_remaining_for_col[index.col_num] -= 1;
        }

        *changed = true;
    }

    // In the given row/col, replace all Unknown squares with the specified value
    pub fn replace_unknown(&mut self, row_or_col: RowOrCol, new_value: Square, changed: &mut bool) {
        for coord in row_or_col.coords() {
            if self[coord] == Square::Unknown {
                self.set(coord, new_value, changed);
            }
        }
    }

    /////////////////////////////////////////////////////////////////////
    //
    // Contents of the board

    // TODO: Should this also ensure that self.remaining_ship_sizes() is empty?
    pub fn is_solved(&self) -> bool {
        self.layout.all_coordinates()
            .all(|coord| self[coord] != Square::Unknown)
    }

    // Count number of ships remaining in the given row/col
    pub fn ships_remaining(&self, row_or_col: RowOrCol) -> usize {
        match row_or_col.axis {
            Axis::Row => self.ships_remaining_for_row[row_or_col.index],
            Axis::Col => self.ships_remaining_for_col[row_or_col.index],
        }
    }

    // Enumerate all the sizes of ships that remain to be found
    pub fn remaining_ship_sizes<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.ships_to_find.iter()
            .filter_map(|(&ship_size, &count)|
                if count > 0 {
                    Some(ship_size)
                }
                else {
                    None
                }
            )
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

    // Count how many ships of a given size are found
    fn count_found_ships(&self, ship_size: usize) -> usize {
        self.layout.possible_coords_for_ship(ship_size)
            .filter(move |(coord, incrementing_axis)| {
                self.ship_exists_at_coord(ship_size, *coord, *incrementing_axis)
            })
            .count()
    }

    // Does a specific ship (size + axis) exist at these coords?
    fn ship_exists_at_coord(&self, ship_size: usize, origin: Coord, incrementing_axis: Axis) -> bool {
        self.layout.coords_in_ship(ship_size, origin, incrementing_axis)
            .enumerate()
            .all(|(square_idx, curr_coord)| {
                let expected = ShipSquare::expected_square_for_ship(ship_size, square_idx, incrementing_axis);
                self[curr_coord] == Square::ShipSquare(expected)                
            })
    }
}

impl<'a> Index<Coord<'a>> for Board {
    type Output = Square;

    fn index(&self, index : Coord) -> &Square {
        &self.squares[index.row_num][index.col_num]
    }
}

#[cfg(test)] use crate::test_utils::*;

#[cfg(test)]
mod test {
    use super::*;

	#[test]
	fn it_sets_changed() {
	    let mut board = Board::new(&vec![
	        "  001",
	        "0|   ",
	        "1|~  ",
	    ]);
        let layout = board.layout;

	    let mut changed = false;

	    // Setting an unchanged square leaves changed alone
	    let coord = layout.coord(0, 1);
	    board.set(coord, Square::Water, &mut changed);
	    assert_eq!(changed, false);

	    // Setting a changed square sets changed
	    let coord = layout.coord(0, 0);
	    board.set(coord, Square::Water, &mut changed);
	    assert_eq!(changed, true);

	    // Once changed is true, don't set it back to false
	    let coord = layout.coord(0, 0);
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
        let layout = board.layout;

        let coord1 = layout.coord(0, 1);

        assert_eq!(board[coord1], Square::Unknown);
        
        let mut _changed = false;
        board.set(coord1, Square::Water, &mut _changed);
        
        assert_eq!(board[coord1], Square::Water);

        let coord2 = layout.coord(3, 1);
        assert_eq!(board[coord2], Square::Water);
    }

    // TODO: This test should move to layout
    #[test]
    fn it_accesses_col() {
        let board = make_test_board();
        // col1 needs to be its own variable, for lifetime reasons
        let col1 = board.layout.col(1);
        let mut coords = col1.coords();

        let expected_coord = board.layout.coord(1, 0);
        assert_eq!(coords.next(), Some(expected_coord));

        let expected_coord = board.layout.coord(1, 1);
        assert_eq!(coords.next(), Some(expected_coord));

        assert_eq!(coords.next(), None);
    }

    #[test]
    fn it_counts_ships_remaining() {
        let board = make_test_board();

        let row0 = board.layout.row(0);
        let row1 = board.layout.row(1);
        let col0 = board.layout.col(0);
        let col2 = board.layout.col(2);

        assert_eq!(board.ships_remaining(row0), 1);
        assert_eq!(board.ships_remaining(row1), 2);
        assert_eq!(board.ships_remaining(col0), 1);
        assert_eq!(board.ships_remaining(col2), 0);     
    }

    #[test]
    fn it_adjusts_ships_remaining_after_set() {
        let mut board = make_test_board();
        let layout = board.layout;
        let coord = layout.coord(0, 1);

        assert_eq!(
            board.ships_remaining(layout.row(coord.row_num)),
            2);

        assert_eq!(
            board.ships_remaining(layout.col(coord.col_num)),
            1);

        let mut changed = false;
        board.set(coord, Square::ShipSquare(ShipSquare::Any), &mut changed);

        // ships remaining has decreased
        assert_eq!(
            board.ships_remaining(layout.row(coord.row_num)),
            2 - 1);
        assert_eq!(
            board.ships_remaining(layout.col(coord.col_num)),
            1 - 1);

        assert_eq!(changed, true);
    }

    // TODO: This test should move to layout
    #[test]
    fn it_accesses_col_contents() {
        let board = Board::new(&vec![
            "  000",
            "0| ^ ",
            "0| | ",
            "0| v ",
            "0| ~ ",
        ]);
        let layout = board.layout;

        let col = layout.col(1);
        let mut col_coords = col.coords();

        let mut expected_coord;

        expected_coord = layout.coord(1, 0);
        assert_eq!(col_coords.next(), Some(expected_coord));
        assert_eq!(board[expected_coord], Square::ShipSquare(ShipSquare::TopEnd));

        expected_coord = layout.coord(1, 1);
        assert_eq!(col_coords.next(), Some(expected_coord));
        assert_eq!(board[expected_coord], Square::ShipSquare(ShipSquare::VerticalMiddle));

        expected_coord = layout.coord(1, 2);
        assert_eq!(col_coords.next(), Some(expected_coord));
        assert_eq!(board[expected_coord], Square::ShipSquare(ShipSquare::BottomEnd));

        expected_coord = layout.coord(1, 3);
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
        let board = Board::new(&vec![
            "  000",
            "0|  ^",
            "0| ~|",
            "0|• |",
            "0|  v",
            "0|   ",
            "0|<> ",
        ]);    

        // Find vertical ship
        let coord = board.layout.coord(2, 0);
        let found_ship = board.ship_exists_at_coord(
            /* size */ 4, coord, Axis::Row
            );
        assert_eq!(found_ship, true);

        // Find horizontal ship
        let coord = board.layout.coord(0, 5);
        let found_ship = board.ship_exists_at_coord(
            /* size */ 2, coord, Axis::Col
            );
        assert_eq!(found_ship, true);        

        // Finds dot
        let coord = board.layout.coord(0, 2);
        let found_ship = board.ship_exists_at_coord(
            /* size */ 1, coord, Axis::Row
            );
        assert_eq!(found_ship, true);        

        // Does not match when size < ship size
        let coord = board.layout.coord(2, 0);
        let found_ship = board.ship_exists_at_coord(
            /* size */ 3, coord, Axis::Row
            );
        assert_eq!(found_ship, false);

        // Does not match when size > ship size.
        // We ask for a ship of size 3, but the ship 
        // has size 4.
        let coord = board.layout.coord(2, 0);
        let found_ship = board.ship_exists_at_coord(
            /* size */ 5, coord, Axis::Row
            );
        assert_eq!(found_ship, false);                
    }

    #[test]
    fn it_counts_ships() {
        let board = Board::new(&vec![
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

