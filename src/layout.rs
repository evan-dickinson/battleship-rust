use std::fmt;

use crate::neighbor::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coord {
    pub row_num : usize,
    pub col_num : usize,
}

impl Coord {
    // Return the row or col of this coord, whichever is specified by the axis
    pub fn row_or_col(&self, axis : Axis) -> RowOrCol {
        RowOrCol { axis, index: self.index_for_axis(axis) }
    }

    pub fn index_for_axis(&self, axis : Axis) -> usize {
        match axis {
            Axis::Row => self.row_num,
            Axis::Col => self.col_num,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    Row,
    Col
}

impl Axis {
    pub fn cross_axis(self) -> Self {
        match self {
            Axis::Row => Axis::Col,
            Axis::Col => Axis::Row,
        }
    }
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = match self {
            Axis::Row => "Row",
            Axis::Col => "Col",
        };

        write!(f, "{}", label)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RowOrCol {
    pub axis : Axis,
    pub index : usize
}

impl fmt::Display for RowOrCol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.axis, self.index)
    }
}

// all methods relating to a board's coordinates, width, and height
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Layout {
    pub num_rows : usize,
    pub num_cols : usize,
}

impl Layout {
    // Return the coord that is the result of moving the eisting coord by `offset` sqares, in the given axis
    pub fn offset(&self, coord: Coord, offset: usize, axis: Axis) -> Option<Coord> {
        let new_coord = match axis {
            Axis::Row => Coord { 
                row_num: coord.row_num + offset, 
                col_num: coord.col_num 
            },
            Axis::Col => Coord { 
                row_num: coord.row_num,  
                col_num: coord.col_num + offset 
            },
        };

        if new_coord.row_num < self.num_rows && new_coord.col_num < self.num_cols {
            Some(new_coord)
        }
        else {
            None
        }
    }

    pub fn all_coordinates(&self) -> impl Iterator<Item = Coord> {
        // Don't want to capture self in any of the closures we return.
        // TODO: Not sure that matters
        let num_rows = self.num_rows;
        let num_cols = self.num_cols;
        let num_squares = num_rows * num_cols;

        (0..num_squares).map(move |idx| {
            Coord {
                row_num: idx / num_cols,
                col_num: idx % num_cols,
            }
        })
    }    

    // Iterate over every row and column in the layout
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

        rows.chain(cols)
    }    

    // Return all the coordinates along the specified row or col
    pub fn coords_for(&self, row_or_col: RowOrCol) -> impl Iterator<Item = Coord>  {
        // Count number of items in the minor axis
        let minor_axis_ubound = match row_or_col.axis {
            Axis::Row => self.num_cols,
            Axis::Col => self.num_rows
        };
        let range = 0 .. minor_axis_ubound;

        let major_axis_idx = row_or_col.index;

        range.map(move |minor_axis_idx| {
            match row_or_col.axis {
                Axis::Row => Coord {
                    row_num: major_axis_idx,
                    col_num: minor_axis_idx,
                },
                Axis::Col => Coord {
                    row_num: minor_axis_idx,
                    col_num: major_axis_idx
                }
            }
        })
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
            Some(Coord {
                row_num: i_row as usize,
                col_num: i_col as usize,
            })
        }
        else {
            None
        }        
    }

    // Given a starting point for a ship, iterate over the coordinates that make up the squares of that ship.
    //
    // Panics: If the ship is not in bounds
    pub fn squares_in_ship<'a>(&'a self, ship_size: usize, origin: Coord, incrementing_axis: Axis) -> impl Iterator<Item = Coord> + 'a {
        (0..ship_size).map(move |square_idx| {
            self.offset(origin, square_idx, incrementing_axis).unwrap()
        })
    }

    // For a ship of a given size, iterate over coordinates (and axies) where a ship like that might be placed.
    // It only returns values that will be in bounds
    pub fn possible_coords_for_ship<'a>(&'a self, ship_size: usize) -> impl Iterator<Item = (Coord, Axis)> + 'a {
        // When placing size = 1, we don't increment the coordinate so axis doesn't matter. But if we
        // search by both axes, every coord will match twice. So only search by one axis, and we only match
        // every candidate coordinate once.
        let axes = if ship_size == 1 { 
            &[Axis::Row][..] // Use [..] to create a slice, not a statically-sized array
        }
        else {
            &[Axis::Row, Axis::Col][..]
        };

        // Make a sequence of iterators. One iterator per axis.
        // Iterator 1: (_, Row), (_, Row), (_, Row)
        // Iterator 2: (_, Col), (_, Col), (_, Col)
        let iterators = axes.into_iter().cloned().map(move |incrementing_axis| {
            self.all_coordinates()
            .map(move |origin| (origin, incrementing_axis) )
            .filter(move |(origin, incrementing_axis)| {
                self.ship_in_bounds(ship_size, *origin, *incrementing_axis)
            })
        });

        // Chain all the iterators together
        // Resulting iterator: (_, Row), (_, Row), ... (_, Col), (_, Col), ...
        iterators.fold(None, |chained_iterators_opt: Option<Box<dyn Iterator<Item = _>>>, curr_iterator| {
            match chained_iterators_opt {
                None => Some(Box::new(curr_iterator)),
                Some(prev_iterators) => Some(Box::new(prev_iterators.chain(curr_iterator)))
            }
        }).unwrap()
    }

    // Would a ship at the given origin fit in bounds?
    fn ship_in_bounds(&self, ship_size: usize, origin: Coord, incrementing_axis: Axis) -> bool {
        let num_squares_in_bounds = (0..ship_size).filter_map(|square_idx| {
            // Offset will return None if it goes out of bounds
            self.offset(origin, square_idx, incrementing_axis)
        })
        .count();

        num_squares_in_bounds == ship_size
    }

}

#[cfg(test)] use crate::test_utils::*;

#[cfg(test)]
mod layout_tests {
    use std::collections::HashSet;
    use super::*;

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

    #[test] 
    fn it_finds_offsets() {
        let layout = Layout { num_rows: 3, num_cols: 4 };

        let coord = Coord { row_num: 1, col_num: 2};

        // Row - In bounds
        let new_coord = layout.offset(coord, 1, Axis::Row).unwrap();
        assert_eq!(new_coord.row_num, 2);
        assert_eq!(new_coord.col_num, 2);

        // Col - in bounds
        let new_coord = layout.offset(coord, 1, Axis::Col).unwrap();
        assert_eq!(new_coord.row_num, 1);
        assert_eq!(new_coord.col_num, 3);  

        // Row - Out of bounds
        let new_coord = layout.offset(coord, 2, Axis::Row);
        assert_eq!(new_coord, None);

        // Row - Out of bounds
        let new_coord = layout.offset(coord, 2, Axis::Col);
        assert_eq!(new_coord, None);        
    }

    #[test]
    fn test_ship_in_bounds() {
        let layout = Layout { num_rows: 3, num_cols: 4 };
        let origin = Coord { row_num: 0, col_num: 1};

        // In bounds
        let in_bounds = layout.ship_in_bounds(3, origin, Axis::Row);
        assert_eq!(in_bounds, true);

        // Out of bounds
        let in_bounds = layout.ship_in_bounds(5, origin, Axis::Col);
        assert_eq!(in_bounds, false);

    }
}
