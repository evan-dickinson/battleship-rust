// rustc --crate-type lib --emit llvm-ir lib.rs -O

use std::ops::Index;
use std::fmt;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::iter::IntoIterator;

pub mod client;
pub mod network;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coord {
    row_num : usize,
    col_num : usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Neighbor {
    N, NE, E, SE, S, SW, W, NW
}

impl Neighbor {
    pub fn all_neighbors() -> [Neighbor; 8] {
        let all_neighbors = [
            Neighbor::N,
            Neighbor::NE,
            Neighbor::E,
            Neighbor::SE,
            Neighbor::S,
            Neighbor::SW,
            Neighbor::W,
            Neighbor::NW,
        ];

        return all_neighbors;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    Row,
    Col
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RowOrCol {
    pub axis : Axis,
    pub index : usize
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Ship {
    Any,
    LeftEnd,
    RightEnd,
    TopEnd,
    BottomEnd,
    VerticalMiddle,
    HorizontalMiddle,
    Dot // single square
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Square {
    Unknown,
    Water,
    Ship(Ship)
}

impl Square {
    fn is_ship(&self) -> bool {
        match self {
            Square::Ship(_) => true,
            _               => false
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char = match self {
            Square::Unknown => ' ',
            Square::Water   => '~',

            Square::Ship(ship_type) => match ship_type {
                Ship::Any              => '*',
                Ship::Dot              => '•',              
                Ship::LeftEnd          => '<',
                Ship::RightEnd         => '>',
                Ship::TopEnd           => '^',
                Ship::BottomEnd        => 'v',
                Ship::VerticalMiddle   => '|',
                Ship::HorizontalMiddle => '-',
            }
        };

        return write!(f, "{}", char)
    }
}

impl From<char> for Square {
    fn from(square_char : char) -> Self {
        return match square_char {
            ' ' => Square::Unknown,
            '~' => Square::Water,
            '*' => Square::Ship(Ship::Any),
            '•' => Square::Ship(Ship::Dot),
            '<' => Square::Ship(Ship::LeftEnd),
            '>' => Square::Ship(Ship::RightEnd),
            '^' => Square::Ship(Ship::TopEnd),
            'v' => Square::Ship(Ship::BottomEnd),
            '|' => Square::Ship(Ship::VerticalMiddle),
            '-' => Square::Ship(Ship::HorizontalMiddle),
            _   => panic!("Unknown char".to_string())
        }       
    }
}

// all methods relating to a board's coordinates, width, and height
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Layout {
    num_rows : usize,
    num_cols : usize,
}

impl Layout {
    pub fn all_coordinates(&self) -> impl Iterator<Item = Coord> {
        // Don't want to capture self in any of the closures we return.
        // TODO: Not sure that matters
        let num_rows = self.num_rows;
        let num_cols = self.num_cols;
        let num_squares = num_rows * num_cols;

        return (0..num_squares).map(move |idx| {
            Coord {
                row_num: idx / num_cols,
                col_num: idx % num_cols,
            }
        })
    }    

    pub fn rows_and_cols(&self) -> impl Iterator<Item = RowOrCol> {
        let rows = (0 .. self.num_rows).map(|row_num| {
            RowOrCol {
                axis: Axis::Row,
                index: row_num
            }
        });

        let cols = (0 .. self.num_cols).map(|col_num| {
            RowOrCol {
                axis: Axis::Col,
                index: col_num
            }
        });     

        return rows.chain(cols);
    }    

    // Return all the coordinates along the specified row or col
    // todo: rename to coordinates_for
    pub fn coordinates(&self, row_or_col : RowOrCol) -> impl Iterator<Item = Coord>  {
        // Count number of items in the minor axis
        let minor_axis_ubound = match row_or_col.axis {
            Axis::Row => self.num_cols,
            Axis::Col => self.num_rows
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

    pub fn coord_for_neighbor(&self, index: Coord,
        neighbor: Neighbor) -> Option<Coord> {
        // convert to signed so we can check for < 0
        let i_num_rows = self.num_rows as isize;
        let i_num_cols = self.num_cols as isize;

        let i_row_num = index.row_num as isize;
        let i_col_num = index.col_num as isize;

        let (i_row, i_col) : (isize, isize) = match neighbor {
            Neighbor::N  => (i_row_num - 1, i_col_num),
            Neighbor::NE => (i_row_num - 1, i_col_num + 1),
            Neighbor::E  => (i_row_num,     i_col_num + 1),
            Neighbor::SE => (i_row_num + 1, i_col_num + 1),
            Neighbor::S  => (i_row_num + 1, i_col_num),
            Neighbor::SW => (i_row_num + 1, i_col_num - 1),
            Neighbor::W  => (i_row_num,     i_col_num - 1),
            Neighbor::NW => (i_row_num - 1, i_col_num - 1),
        };

        let in_bounds = 
            i_row >= 0         && i_col >= 0 &&
            i_row < i_num_rows && i_col < i_num_cols;

        if in_bounds {
            return Some(Coord {
                row_num: i_row as usize,
                col_num: i_col as usize,
            });
        }
        else {
            return None;
        }        
    }

    pub fn coords_for_neighbors<'a>(&'a self, 
            index: Coord, 
            neighbors: impl IntoIterator<Item = &'a Neighbor> + 'a)
        -> impl Iterator<Item = Coord> + 'a
        {

        return neighbors.into_iter()
        .filter_map(move |neighbor| {
            self.coord_for_neighbor(index, *neighbor)
        });
    }    
}

pub struct Board {
    squares: Vec<Vec<Square>>,
    ships_remaining_for_col: Vec<usize>,
    ships_remaining_for_row: Vec<usize>,
    layout : Layout,
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

    pub fn set(&mut self, index : Coord, value : Square) {
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
    }

    pub fn set_bulk(&mut self, indexes : &mut Iterator<Item = Coord>, value : Square) {
        indexes.for_each(|index| {
            self.set(index, value);
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
    pub fn replace_unknown(&mut self, row_or_col : RowOrCol, new_value : Square) {
        for coord in self.layout.coordinates(row_or_col) {
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
        for row_or_col in board.layout.rows_and_cols() {
            if board.ships_remaining(row_or_col) == 0 {
                board.replace_unknown(row_or_col, Square::Water);
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

    // If number of Unknown squares on an axis == number of ships unaccounted for,
    // fill the blank spots with ships
    fn fill_with_ships(board: &mut Board) {
        for row_or_col in board.layout.rows_and_cols() {
            // Count unknown squares on this row or col
            let num_unknown = board.layout.coordinates(row_or_col)
                .filter(|coord| { board[*coord] == Square::Unknown } )
                .count();               

            if num_unknown == board.ships_remaining(row_or_col) {
                board.replace_unknown(row_or_col, Square::Ship(Ship::Any));
            }
        }
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

    fn surround_dots_with_water(board: &mut Board) {
        let layout = board.layout;
        let ship_coords = {
            layout.all_coordinates()
                .filter(|coord| { 
                    board[*coord] == Square::Ship(Ship::Dot)
                })
                .collect::<Vec<_>>()
        };

        for coord in ship_coords {
            let neighbors = Neighbor::all_neighbors();
            let mut neighbor_coords = layout.coords_for_neighbors(coord, neighbors.iter());
            board.set_bulk(&mut neighbor_coords, Square::Water);
        }
    }

    #[test]
    fn it_surrounds_dots() {
        let mut board = Board::new(vec![
            "  00000",
            "0|     ",
            "0|  •  ",
            "0|     ",
        ]);

        surround_dots_with_water(&mut board);
        let expected = vec![
            "  00000",
            "0| ~~~ ",
            "0| ~•~ ",
            "0| ~~~ ",
        ].iter().map(|x| x.to_string()).collect::<Vec<_>>();
        assert_eq!(board.to_strings(), expected);        
    }

    fn surround_ends_with_water(board: &mut Board) {
       let all_neighbors = Neighbor::all_neighbors();

        let ends = [
            // (a, b)
            // when you find a, fill in all neighbours except b
            (Ship::LeftEnd, Neighbor::E),
            (Ship::RightEnd, Neighbor::W),
            (Ship::TopEnd, Neighbor::S),
            (Ship::BottomEnd, Neighbor::N),
        ];

        let layout = board.layout;

        let neighbors_of_ships = ends.iter()
            // find all coords containing a ship of type end_type
            .map(|(end_type, ignore_neighbor)| {
                let nc = all_neighbors.clone();

                layout.all_coordinates()
                    .filter(|coord| {
                        board[*coord] == Square::Ship(*end_type)
                    })
                    .map(|coord| {
                        let neighbors = nc.into_iter()
                            .filter(|&n| { *n != *ignore_neighbor });

                        layout.coords_for_neighbors(coord, neighbors)
                            .collect::<Vec<Coord>>()
                    })
                    .fold(vec![], move |mut acc, mut curr_vec| {
                        acc.append(&mut curr_vec);
                        acc
                    })
            })
            .collect::<Vec<Vec<Coord>>>();

        for neighbor_coords in neighbors_of_ships {
            for coord in neighbor_coords {
                board.set(coord, Square::Water);
            }            
        }
    }

    fn surround_middles_with_water(board: &mut Board) {
        let layout = board.layout;
        let ship_coords = {
            layout.all_coordinates()
                .filter(|coord| { 
                    board[*coord] == Square::Ship(Ship::VerticalMiddle) ||
                    board[*coord] == Square::Ship(Ship::HorizontalMiddle)
                })
                .collect::<Vec<_>>()
        };

        for coord in ship_coords {
            let neighbors = match board[coord] {
                Square::Ship(Ship::VerticalMiddle) => [
                    Neighbor::NE, Neighbor::E, Neighbor::SE,
                    Neighbor::NW, Neighbor::W, Neighbor::SW,
                ],
                Square::Ship(Ship::HorizontalMiddle) => [
                    Neighbor::NW, Neighbor::N, Neighbor::NE,
                    Neighbor::SW, Neighbor::S, Neighbor::SE,
                ],
                _   => panic!("Should not happen"),
            };


            let mut neighbor_coords = layout.coords_for_neighbors(coord, neighbors.iter());
            board.set_bulk(&mut neighbor_coords, Square::Water);
        }
    }


    fn fill_diagonals_with_water(board: &mut Board) {
        let diagonals = [
            Neighbor::NE,
            Neighbor::SE,
            Neighbor::NW,
            Neighbor::SW
        ];

        let layout = board.layout;

        let ship_coords = {
            layout.all_coordinates()
                .filter(|coord| { board[*coord].is_ship() })
                .collect::<Vec<_>>()
        };
        for coord in ship_coords {
            let mut neighbor_coords = 
                layout.coords_for_neighbors(coord, diagonals.iter());

            board.set_bulk(&mut neighbor_coords, Square::Water);
        }
    }

    #[test]
    fn it_fills_diagonals() {
        let mut board = Board::new(vec![
            "  00000",
            "0|     ",
            "0|     ",
            "0|  *  ",
            "0|     ",
            "0|     ",
        ]);

        fill_diagonals_with_water (&mut board);
        let expected = vec![
            "  00000",
            "0|     ",
            "0| ~ ~ ",
            "0|  *  ",
            "0| ~ ~ ",
            "0|     ",        
        ].iter().map(|x| x.to_string()).collect::<Vec<_>>();
        assert_eq!(board.to_strings(), expected);
    }    

    fn place_ships_next_to_ends(board: &mut Board) {
        let layout = board.layout;
        let ship_coords = {
            layout.all_coordinates()
                .filter(|coord| { board[*coord].is_ship() })
                .collect::<Vec<_>>()
        };
        for coord in ship_coords {
            let neighbor = match board[coord] {
                Square::Ship(Ship::TopEnd) => Some(Neighbor::S),
                Square::Ship(Ship::BottomEnd) => Some(Neighbor::N),
                Square::Ship(Ship::LeftEnd) => Some(Neighbor::E),
                Square::Ship(Ship::RightEnd) => Some(Neighbor::W),
                _ => None,
            };

            if let Some(neighbor) = neighbor {
                if let Some(neighbor_coord) = 
                    layout.coord_for_neighbor(coord, neighbor)  {

                    board.set(neighbor_coord, Square::Ship(Ship::Any));
                }
            }
        }        
    }

    #[test]
    fn it_places_ships_next_to_ends() {
        let mut board = Board::new(vec![
            "  00100",
            "0|  ^  ",
            "1|     ",
        ]);

        place_ships_next_to_ends(&mut board);
        let expected = vec![
            "  00000",
            "0|  ^  ",
            "0|  *  ",    
        ].iter().map(|x| x.to_string()).collect::<Vec<_>>();
        assert_eq!(board.to_strings(), expected);
    }

    // TODO: Checks to implement:
    // - Unify the "fill with water" functions
    //   + Can handle all the separate checks with 1 algorithm
    // - Convert "any" to specific ships:
    //   - to dot, when fully surrounded
    //   - to end, when surrounded by water and/or edge of board
    //   - to vert middle, when surrounded by water on left/right
    //   - to horz middle, when surrounded by water on top/bottom
    //   - to generic middle, when surrounded by diagonals
    //     + check for edge of board, too, not just surrounded by water
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

        board.set(coord, Square::Ship(Ship::Any));

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
    fn it_returns_all_coordinates() {
        let board = make_test_board();
        let coords : HashSet<_> = board.layout.all_coordinates().collect();

        assert_eq!(coords.len(), 8);
        let expected_coords : Vec<_> = [
            /* x, y */
            (0, 0), (0, 1), (0, 2), (0usize, 3usize),
            (1, 0), (1, 1), (1, 2), (1, 3),
        ].iter()
        .map(|(x, y)| { Coord { row_num: *x, col_num: *y } })
        .collect();

        println!("coords: {:?}", coords);

        for expected in expected_coords {
            assert!(coords.contains(&expected), 
                "Should have contained {:?}", expected);
        }
    }
}



