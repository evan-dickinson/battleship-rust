use std::ops::Index;
use std::collections::HashMap;

use crate::error::*;
use crate::layout::*;
use crate::parse::*;
use crate::ship::*;
use crate::square::*;

pub struct Board {
    ships_to_find: HashMap<ExpectedShip, usize>, // ExpectedShip => count of ships remaining
    squares: Vec<Vec<Square>>,
    ship_squares_remaining_for_col: Vec<usize>,
    ship_squares_remaining_for_row: Vec<usize>,
    dirty: bool,
    pub layout: Layout,
}

impl Board {
    /////////////////////////////////////////////////////////////////////
    //
    // Board creation

    pub fn new(text_lines : &[&str]) -> Result<Self> {        
        let mut text = text_lines.join("\n");
        text.push_str("\n.");

        parse_board(&text)
    }

    pub fn new_from_data(squares: Vec<Vec<Square>>, 
        ship_squares_remaining_for_row: Vec<usize>,
        ship_squares_remaining_for_col: Vec<usize>,
        ships_to_find: HashMap<usize, usize>) -> Self {

        let layout = Layout {
            num_rows: squares.len(),
            num_cols: squares[0].len(),            
        };

        let mut board = Board {
            squares,
            ship_squares_remaining_for_col,
            ship_squares_remaining_for_row,
            layout,
            dirty: false,
            ships_to_find: Default::default()
        };

        // Convert ships_to_find 
        // - from: usize        => usize
        // - to:   ExpectedShip => usize
        let to_find_iter = ships_to_find
            .iter()
            .map(|(&size, &count)| {
                let expected_ship = ExpectedShip { size };

                (expected_ship, count)
            });
        board.ships_to_find.extend(to_find_iter);

        board
    }

    /////////////////////////////////////////////////////////////////////
    //
    // Printing / converting to string

    fn format_col_headers(&self) -> String {
        let prefix = "  ".to_string(); // start the line with two blanks
        self.ship_squares_remaining_for_col.iter()
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
                let row_count = self.ship_squares_remaining_for_row[row_num];
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

        let mut expected_ships = self.ships_to_find.keys()
            //.cloned()
            .collect::<Vec<_>>();

        expected_ships.sort();  
        expected_ships.reverse();

        let ship_strings = expected_ships.iter()
            .map(|&expected_ship| {
                let count = self.num_remaining_ships_to_find(*expected_ship);
                let msg = format!("{} x {}", expected_ship, count);
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
    // Change tracking

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false
    }

    /////////////////////////////////////////////////////////////////////
    //
    // Setting values


    // changed: set to true if board[index] != value, othewise do not set
    pub fn set(&mut self, index: Coord, new_value: Square) -> Result<()> {
        let curr_value = self[index];

        if curr_value == new_value {
            return Ok(());
        }

        // Check for logic errors. You can only:
        // - Refine ShipSquare::Any to a more specific kind of ship
        // - Refine ShipSquare::AnyMiddle to a more specific kind of middle
        // - Change Unknown to another value
        if curr_value == Square::ShipSquare(ShipSquare::Any) {
            ensure!(new_value.is_ship(), 
                "Attempting to set {:?} to a non-ship value: {:?}. index: {:?}",
                curr_value, new_value, index);
        }
        else if curr_value == Square::ShipSquare(ShipSquare::AnyMiddle) {
            ensure!(new_value.is_ship_middle(), 
                "Attempting to set {:?} to a non-middle value: {:?}. index: {:?}",
                curr_value, new_value, index);
        }
        else {
            ensure!(curr_value == Square::Unknown,
                "Attempting to set a square whose current value is not Unknown. new_value: {:?} index: {:?}",
                new_value, index);
        }

        self.squares[index.row_num][index.col_num] = new_value;

        // Update ships remaining
        if new_value.is_ship() && !curr_value.is_ship() {
            self.ship_squares_remaining_for_row[index.row_num] -= 1;
            self.ship_squares_remaining_for_col[index.col_num] -= 1;
        }
        
        self.dirty = true;
        Ok(())
    }

    // In the given row/col, replace all Unknown squares with the specified value
    pub fn replace_unknown(&mut self, row_or_col: RowOrCol, new_value: Square) -> Result<()> {
        for coord in row_or_col.coords() {
            if self[coord] == Square::Unknown { 
                self.set(coord, new_value)?
            }
        }

        Ok(())
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
    pub fn ship_squares_remaining(&self, row_or_col: RowOrCol) -> usize {
        match row_or_col.axis {
            Axis::Row => self.ship_squares_remaining_for_row[row_or_col.index],
            Axis::Col => self.ship_squares_remaining_for_col[row_or_col.index],
        }
    }

    // Enumerate all the sizes of ships that remain to be found
    pub fn remaining_expected_ships<'a>(&'a self) -> impl Iterator<Item = ExpectedShip> + 'a {
        self.ships_to_find.iter()
            .filter_map(|(&expected_ship, &count)|
                if count > 0 {
                    Some(expected_ship)
                }
                else {
                    None
                }
            )
    }

    // How many ships of a given size remain to be found
    pub fn num_remaining_ships_to_find(&self, expected_ship: ExpectedShip) -> usize {
        if let Some(&total) = self.ships_to_find.get(&expected_ship) {
            let found = self.count_found_ships(expected_ship);

            total - found
        }
        else {
            0
        }
    }

    // Count how many ships of a given size are found
    fn count_found_ships(&self, expected_ship: ExpectedShip) -> usize {
        self.layout.possible_heads_for_ship(expected_ship)
            .filter(move |&ship_head| {
                let ship = ship_head.to_ship(expected_ship);
                self.ship_is_found(ship)
            })
            .count()

    }

    // Does a specific ship (size + axis) exist at these coords?
    fn ship_is_found(&self, ship: Ship) -> bool {
        match ship.coords() {
            None         => false, // ship would be out of bounds
            Some(coords) => coords
                .enumerate()
                .all(|(square_idx, curr_coord)| {
                    let expected = ship.expected_square_for_idx(square_idx);
                    self[curr_coord] == Square::ShipSquare(expected)                
                })
        }
    }
}

impl<'coord> Index<Coord<'coord>> for Board {
    type Output = Square;

    fn index(&self, index: Coord) -> &Square {
        &self.squares[index.row_num][index.col_num]
    }
}

#[cfg(test)] use crate::test_utils::*;

#[cfg(test)]
mod test {
    use super::*;

	#[test]
	fn it_sets_changed() -> Result<()> {
	    let mut board = Board::new(&vec![
	        "  001",
	        "0|   ",
	        "1|~  ",
	    ])?;
        let layout = board.layout;

	    // Set a squre to its current value => dirty is false
	    let coord = layout.coord(0, 1);
	    board.set(coord, Square::Water)?;
	    assert_eq!(board.dirty, false);

	    // Set a square to a new value => dirty is true
	    let coord = layout.coord(0, 0);
	    board.set(coord, Square::Water)?;
	    assert_eq!(board.dirty, true);

        Ok(())
	}

    #[test]
    fn it_returns_num_rows() -> Result<()> {
        let board = make_test_board()?;
        assert_eq!(board.layout.num_rows, 2);

        Ok(())
    }

    #[test]
    fn it_returns_num_cols() -> Result<()> {
        let board = make_test_board()?;
        assert_eq!(board.layout.num_cols, 4);

        Ok(())
    }    

    #[test]
    fn it_accesses_with_index() -> Result<()> {
        let mut board = make_test_board()?;
        let layout = board.layout;

        let coord1 = layout.coord(0, 1);

        assert_eq!(board[coord1], Square::Unknown);
        
        board.set(coord1, Square::Water)?;
        
        assert_eq!(board[coord1], Square::Water);
        assert_eq!(board.dirty, true);

        let coord2 = layout.coord(3, 1);
        assert_eq!(board[coord2], Square::Water);

        Ok(())
    }

    // TODO: This test should move to layout
    #[test]
    fn it_accesses_col() -> Result<()> {
        let board = make_test_board()?;
        // col1 needs to be its own variable, for lifetime reasons
        let col1 = board.layout.col(1);
        let mut coords = col1.coords();

        let expected_coord = board.layout.coord(1, 0);
        assert_eq!(coords.next(), Some(expected_coord));

        let expected_coord = board.layout.coord(1, 1);
        assert_eq!(coords.next(), Some(expected_coord));

        assert_eq!(coords.next(), None);

        Ok(())
    }

    #[test]
    fn it_counts_ship_squares_remaining() -> Result<()> {
        let board = Board::new(&vec![
            "  0123", 
            "9|    ", 
            "8|    ",
            "7|    ",
        ])?;

        let row0 = board.layout.row(0);
        let row1 = board.layout.row(1);
        let col0 = board.layout.col(0);
        let col2 = board.layout.col(2);

        assert_eq!(board.ship_squares_remaining(row0), 9);
        assert_eq!(board.ship_squares_remaining(row1), 8);
        assert_eq!(board.ship_squares_remaining(col0), 0);
        assert_eq!(board.ship_squares_remaining(col2), 2);     

        Ok(())
    }

    #[test]
    fn it_adjusts_ship_squares_remaining_after_set() -> Result<()> {
        let mut board = make_test_board()?;
        let layout = board.layout;
        let coord = layout.coord(0, 1);

        assert_eq!(
            board.ship_squares_remaining(layout.row(coord.row_num)),
            2);

        assert_eq!(
            board.ship_squares_remaining(layout.col(coord.col_num)),
            1);

        board.set(coord, Square::ShipSquare(ShipSquare::Any))?;

        // ships remaining has decreased
        assert_eq!(
            board.ship_squares_remaining(layout.row(coord.row_num)),
            2 - 1);
        assert_eq!(
            board.ship_squares_remaining(layout.col(coord.col_num)),
            1 - 1);

        assert_eq!(board.dirty, true);

        Ok(())
    }

    // TODO: This test should move to layout
    #[test]
    fn it_accesses_col_contents() -> Result<()> {
        let board = Board::new(&vec![
            "  000",
            "0| ^ ",
            "0| | ",
            "0| v ",
            "0| ~ ",
        ])?;
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

        Ok(())
    }
}

#[cfg(test)]
mod test_find_ships {
    use super::*;

    #[test]
    fn it_finds_ship() -> Result<()> {
        let board = Board::new(&vec![
            "  000",
            "0|  ^",
            "0| ~|",
            "0|• |",
            "0|  v",
            "0|   ",
            "0|<> ",
        ])?;

        // Find vertical ship
        let origin = board.layout.coord(2, 0);
        let found_ship = board.ship_is_found(
            Ship { size: 4, head: ShipHead {origin, incrementing_axis: Axis::Row }}
            );
        assert_eq!(found_ship, true);

        // Find horizontal ship
        let origin = board.layout.coord(0, 5);
        let found_ship = board.ship_is_found(
            Ship { size: 2, head: ShipHead {origin, incrementing_axis: Axis::Col }}
            );
        assert_eq!(found_ship, true);        

        // Finds dot
        let origin = board.layout.coord(0, 2);
        let found_ship = board.ship_is_found(
            Ship { size: 1, head: ShipHead {origin, incrementing_axis: Axis::Row }}
            );
        assert_eq!(found_ship, true);        

        // Does not match when size < ship size
        let origin = board.layout.coord(2, 0);
        let found_ship = board.ship_is_found(
            Ship { size: 3, head: ShipHead {origin, incrementing_axis: Axis::Row }}
            );
        assert_eq!(found_ship, false);

        // Does not match when size > ship size.
        // We ask for a ship of size 3, but the ship 
        // has size 4.
        let origin = board.layout.coord(2, 0);
        let found_ship = board.ship_is_found(
            Ship { size: 5, head: ShipHead {origin, incrementing_axis: Axis::Row }}
            );
        assert_eq!(found_ship, false);     

        Ok(())           
    }

    #[test]
    fn it_counts_ships() -> Result<()> {
        let board = Board::new(&vec![
            "  00000",
            "0|  ^ ^",
            "0| ~| |",
            "0|• | |",
            "0|  v v",
            "0|     ",
        ])?;

        let count = board.count_found_ships(1.into());
        assert_eq!(count, 1);

        let count = board.count_found_ships(3.into());
        assert_eq!(count, 0);    
        
        let count = board.count_found_ships(4.into());
        assert_eq!(count, 2);      

        let count = board.count_found_ships(5.into());
        assert_eq!(count, 0);        

        Ok(())      
    }
}

