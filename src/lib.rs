// rustc --crate-type lib --emit llvm-ir lib.rs -O

// use std::collections::HashSet;
// use std::iter::FromIterator;
// use std::iter::IntoIterator;

mod neighbor;
use self::neighbor::*;

mod square;
use self::square::*;

mod layout;
use self::layout::*;

mod board;
use self::board::*;

mod test_utils;
use self::test_utils::*;

mod solve;
use self::solve::*;

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn it_returns_num_rows() {
        let board = test_utils::make_test_board();
        assert_eq!(board.layout.num_rows, 2);
    }

    #[test]
    fn it_returns_num_cols() {
        let board = test_utils::make_test_board();
        assert_eq!(board.layout.num_cols, 4);
    }    

    #[test]
    fn it_accesses_with_index() {
        let mut board = test_utils::make_test_board();
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
        let board = test_utils::make_test_board();
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
        let board = test_utils::make_test_board();

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
        let mut board = test_utils::make_test_board();
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
}



