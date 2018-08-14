use std::fmt;

use neighbor::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coord {
    pub row_num : usize,
    pub col_num : usize,
}

impl Coord {
    // Return the row or col of this coord, whichever is specified by the axis
    pub fn row_or_col(&self, axis : Axis) -> RowOrCol {
        return RowOrCol { axis, index: self.index_for_axis(axis) };
    }

    pub fn index_for_axis(&self, axis : Axis) -> usize {
        return match axis {
            Axis::Row => self.row_num,
            Axis::Col => self.col_num,
        };
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    Row,
    Col
}

impl Axis {
    pub fn cross_axis(&self) -> Self {
        return match self {
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

        return write!(f, "{}", label);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RowOrCol {
    pub axis : Axis,
    pub index : usize
}

impl fmt::Display for RowOrCol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}: {}", self.axis, self.index);
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
    pub fn offset(&self, coord : Coord, offset: usize, axis: Axis) -> Option<Coord> {
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
            return Some(new_coord);
        }
        else {
            return None;
        }
    }

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

#[cfg(test)] use test_utils::*;

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
}
