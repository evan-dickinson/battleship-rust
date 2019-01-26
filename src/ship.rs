use std::fmt;

use crate::layout::*;
use crate::square::*;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub struct ExpectedShip {
    pub size: usize,
}

impl ExpectedShip {
    // Iterate over indexes from 0 .. size
    pub fn square_indexes(&self) -> impl Iterator<Item = usize> {
        (0 .. self.size).into_iter()
    }
}

impl fmt::Display for ExpectedShip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}sq", self.size)
    }
}

impl From<usize> for ExpectedShip {
    fn from(size: usize) -> Self {
        ExpectedShip { size }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct ShipHead<'a> {
    pub origin: Coord<'a>,

    // constant axis: The one that remains the same as we increment through coordinats
    // incrementing axis: The one that changes as we increment through coordinates
    pub incrementing_axis: Axis,
}

impl<'a> ShipHead<'a> {
    pub fn to_ship(self, expected_ship: ExpectedShip) -> Ship<'a> {
        Ship {
            head: self,
            size: expected_ship.size
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Ship<'a> {
    pub head: ShipHead<'a>,
    pub size: usize,
}

impl<'a> Ship<'a> {    
    #[allow(dead_code)] // currently, this is only used in tests    
    pub fn new(origin: Coord<'a>, incrementing_axis: Axis, size: usize) -> Self {
        Ship {
            head: ShipHead {
                origin,
                incrementing_axis
            },
            size
        }
    }

    // Iterate over indexes from 0 .. size
    pub fn square_indexes(&self) -> impl Iterator<Item = usize> {
        (0 .. self.size).into_iter()
    }

    pub fn is_in_bounds(&self) -> bool {
        let last_square_idx = self.size - 1;
        let last_square = self.head.origin.offset(last_square_idx, self.head.incrementing_axis);
        let is_in_bounds = last_square.is_some();

        is_in_bounds
    }

    // If a ship with this size & origin would be in bounds, return an iterator of the coordinates.
    // If the ship would go out of bounds, return None
    pub fn coords(&'a self) -> Option<impl Iterator<Item = Coord<'a>> + 'a> {
        if self.is_in_bounds() {
            let iter = self.square_indexes()
                .map(move |square_idx| { 
                    self.head.origin
                        .offset(square_idx, self.head.incrementing_axis)
                        .unwrap()
                });
            Some(iter)
        }
        else {
            None
        }
    }

    // Return the nth square for a ship, along the given axis.
    // For example, a ship of size 3 on horizontal axis, we expect to see LeftEnd, then HorizontalMiddle, then RightEnd    
    pub fn expected_square_for_idx(&self, square_idx: usize) -> ShipSquare {
        assert!(square_idx < self.size);

        if self.size == 1 {
            ShipSquare::Dot
        }
        else {
            enum Position { Start, Middle, End };
            let pos = if square_idx == 0             { Position::Start  }
                else  if square_idx == self.size - 1 { Position::End    }
                else                                 { Position::Middle };

            match (pos, self.head.incrementing_axis) {
                (Position::Start,  Axis::Col) => ShipSquare::LeftEnd,
                (Position::Start,  Axis::Row) => ShipSquare::TopEnd,
                (Position::Middle, Axis::Col) => ShipSquare::HorizontalMiddle,
                (Position::Middle, Axis::Row) => ShipSquare::VerticalMiddle,
                (Position::End,    Axis::Col) => ShipSquare::RightEnd,
                (Position::End,    Axis::Row) => ShipSquare::BottomEnd,
            }
        }
    }    
}

#[cfg(test)]
mod layout_tests {
    use super::*;

    #[test]
    fn test_ship_in_bounds() {
        let layout = Layout { num_rows: 3, num_cols: 4 };
        let origin = layout.coord(1, 0);


        // In bounds
        let ship = Ship {
            head: ShipHead {
                origin, incrementing_axis: Axis::Row
            },
            size: 3
        };
        assert_eq!(ship.is_in_bounds(), true);

        // Out of bounds
        let ship = Ship {
            head: ShipHead {
                origin, incrementing_axis: Axis::Col
            },
            size: 5
        };
        assert_eq!(ship.is_in_bounds(), false);

    }
}    
