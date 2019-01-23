use std::fmt;

use crate::neighbor::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coord<'a> {
    pub row_num: usize,
    pub col_num: usize,

    layout: &'a Layout,
}

impl<'a> Coord<'a> {
    // Return the row or col of this coord, whichever is specified by the axis
    pub fn row_or_col(&self, axis : Axis) -> RowOrCol {
        let index = self.index_for_axis(axis);
        self.layout.row_or_col(axis, index)
    }

    pub fn index_for_axis(&self, axis: Axis) -> usize {
        match axis {
            Axis::Row => self.row_num,
            Axis::Col => self.col_num,
        }
    }

    // Return the coord that is the result of moving self by `offset` sqares, in the given axis
    pub fn offset(&self, offset: usize, axis: Axis) -> Option<Self> {
        let new_coord = match axis {
            Axis::Row => self.layout.coord(self.col_num,          self.row_num + offset),
            Axis::Col => self.layout.coord(self.col_num + offset, self.row_num),
        };

        if new_coord.row_num < self.layout.num_rows 
            && new_coord.col_num < self.layout.num_cols {

            Some(new_coord)
        }
        else {
            None
        }
    }

    pub fn neighbor(&self, neighbor: Neighbor) -> Option<Self> {
        // convert to signed so we can check for < 0
        let i_num_rows = self.layout.num_rows as isize;
        let i_num_cols = self.layout.num_cols as isize;

        let i_row_num = self.row_num as isize;
        let i_col_num = self.col_num as isize;

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
            Some(self.layout.coord(i_col as usize, i_row as usize))
        }
        else {
            None
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
pub struct RowOrCol<'b> {
    pub axis: Axis,
    pub index: usize,

    layout: &'b Layout,
}

// TODO: What's the relationship between this lifetime and lifetimes specified in the 
// function signatures?
impl<'b> RowOrCol<'b> {
    // Return all the coordinates along the specified row or col
    pub fn coords(&self) -> impl Iterator<Item = Coord> {
        // Count number of items in the minor axis
        let minor_axis_ubound = match self.axis {
            Axis::Row => self.layout.num_cols,
            Axis::Col => self.layout.num_rows
        };
        let range = 0 .. minor_axis_ubound;

        let major_axis_idx = self.index;

        range.map(move |minor_axis_idx| {
            let (col_num, row_num) = match self.axis {
                Axis::Row => (minor_axis_idx, major_axis_idx),
                Axis::Col => (major_axis_idx, minor_axis_idx),
            };

            self.layout.coord(col_num, row_num)
        })
    }       
}

impl<'b> fmt::Display for RowOrCol<'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.axis, self.index)
    }
}

// all methods relating to a board's coordinates, width, and height
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Layout {
    pub num_rows : usize,
    pub num_cols : usize,
}

impl Layout {
    pub fn coord(&self, col_num: usize, row_num: usize) -> Coord {
        Coord {
            row_num, 
            col_num,
            layout: &self,
        }
    }

    pub fn row_or_col(&self, axis: Axis, index: usize) -> RowOrCol {
        RowOrCol {
            axis,
            index,
            layout: &self,
        }
    }

    pub fn row(&self, index: usize) -> RowOrCol {
        self.row_or_col(Axis::Row, index)
    }

    pub fn col(&self, index: usize) -> RowOrCol {
        self.row_or_col(Axis::Col, index)
    }

    pub fn all_coordinates<'a>(&'a self) -> impl Iterator<Item = Coord> + 'a {
        // Don't want to capture self in any of the closures we return.
        // TODO: Not sure that matters
        let num_rows = self.num_rows;
        let num_cols = self.num_cols;
        let num_squares = num_rows * num_cols;

        (0..num_squares).map(move |idx| {
            self.coord(
                idx % num_cols, // col_num
                idx / num_cols  // row_num
            )
        })
    }    

    // Iterate over every row and column in the layout
    pub fn rows_and_cols<'a>(&'a self) -> impl Iterator<Item = RowOrCol> + 'a {
        let rows = (0 .. self.num_rows)
            .map(move |row_num| self.row_or_col(Axis::Row, row_num) );

        let cols = (0 .. self.num_cols)
            .map(move |col_num| self.row_or_col(Axis::Col, col_num) );

        rows.chain(cols)
    } 

    // Given a starting point for a ship, iterate over the coordinates that make up the squares of that ship.
    //
    // Panics: If the ship is not in bounds
    pub fn coords_in_ship<'a>(&'a self, ship_size: usize, origin: Coord<'a>, incrementing_axis: Axis) -> impl Iterator<Item = Coord> + 'a {
        (0..ship_size).map(move |square_idx| {
            origin.offset(square_idx, incrementing_axis).unwrap()
        })
    }

    // For a ship of a given size, iterate over coordinates (and axies) where a ship like that might be placed.
    // This ignores the board contents -- coordinates are returned based solely on whether or not the coordinates
    // would put the ship out of bounds.
    // 
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

        axes.into_iter()
            .cloned()
            .map(move |incrementing_axis| {
                // For each incrementing_axis, produce an iterator that generates
                // possible origins along that axis.
                // Iterator 1: (_, Row), (_, Row), (_, Row)
                // Iterator 2: (_, Col), (_, Col), (_, Col)

                self.all_coordinates()
                    .filter_map(move |origin| {
                        if self.ship_in_bounds(ship_size, origin, incrementing_axis) {
                            Some((origin, incrementing_axis))
                        }
                        else {
                            None
                        }
                    })
            })
            .flatten()
    }

    // Would a ship at the given origin fit in bounds?
    // TODO: Usually origin comes before ship_size
    fn ship_in_bounds(&self, ship_size: usize, origin: Coord, incrementing_axis: Axis) -> bool {
        let num_squares_in_bounds = (0..ship_size).filter_map(|square_idx| {
            // Offset will return None if it goes out of bounds
            origin.offset(square_idx, incrementing_axis)
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
        // TODO: I think the test data is wrong. Had to swap X and Y
        // when calling .coord().
        .map(|(x, y)| board.layout.coord(*y, *x) )
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
        let coord  = layout.coord(2, 1);

        // Row - In bounds
        let new_coord = coord.offset(1, Axis::Row).unwrap();
        assert_eq!(new_coord.row_num, 2);
        assert_eq!(new_coord.col_num, 2);

        // Col - in bounds
        let new_coord = coord.offset(1, Axis::Col).unwrap();
        assert_eq!(new_coord.row_num, 1);
        assert_eq!(new_coord.col_num, 3);  

        // Row - Out of bounds
        let new_coord = coord.offset(2, Axis::Row);
        assert_eq!(new_coord, None);

        // Row - Out of bounds
        let new_coord = coord.offset(2, Axis::Col);
        assert_eq!(new_coord, None);        
    }

    #[test]
    fn test_ship_in_bounds() {
        let layout = Layout { num_rows: 3, num_cols: 4 };
        let origin = layout.coord(1, 0);

        // In bounds
        let in_bounds = layout.ship_in_bounds(3, origin, Axis::Row);
        assert_eq!(in_bounds, true);

        // Out of bounds
        let in_bounds = layout.ship_in_bounds(5, origin, Axis::Col);
        assert_eq!(in_bounds, false);

    }
}
