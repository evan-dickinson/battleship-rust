use std::fmt;
use std::collections::HashSet;

use crate::layout::*;
use crate::neighbor::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Ship {
    Any,
    LeftEnd,
    RightEnd,
    TopEnd,
    BottomEnd,
    VerticalMiddle,
    HorizontalMiddle,
    AnyMiddle,
    Dot // single square
}

impl Ship {
    pub fn all() -> impl Iterator<Item = Ship> {
        [
            Ship::Any,
            Ship::LeftEnd,
            Ship::RightEnd,
            Ship::TopEnd,
            Ship::BottomEnd,
            Ship::VerticalMiddle,
            Ship::HorizontalMiddle,
            Ship::AnyMiddle,
            Ship::Dot
        ].into_iter().cloned()
    }

    // Return the nth square for a ship, along the given axis.
    // For example, a ship of size 3 on horizontal axis, we expect to see LeftEnd, then HorizontalMiddle, then RightEnd
    pub fn expected_square_for_ship(ship_size: usize, square_idx: usize, incrementing_axis: Axis) -> Ship {
        assert!(square_idx < ship_size);

        if ship_size == 1 {
            Ship::Dot
        }
        else {
            enum Position { Start, Middle, End };
            let pos = if square_idx == 0             { Position::Start  }
                else  if square_idx == ship_size - 1 { Position::End    }
                else                                 { Position::Middle };

            match (pos, incrementing_axis) {
                (Position::Start,  Axis::Col) => Ship::LeftEnd,
                (Position::Start,  Axis::Row) => Ship::TopEnd,
                (Position::Middle, Axis::Col) => Ship::HorizontalMiddle,
                (Position::Middle, Axis::Row) => Ship::VerticalMiddle,
                (Position::End,    Axis::Col) => Ship::RightEnd,
                (Position::End,    Axis::Row) => Ship::BottomEnd,
            }
        }
    }    

    // For a given ship type, which neigbors should be set to water
    pub fn water_neighbors(&self) -> HashSet<Neighbor> {
        match *self {
            Ship::Any       => [
                Neighbor::NW, Neighbor::NE,
                Neighbor::SW, Neighbor::SE,
            ].into_iter().cloned().collect(),

            Ship::Dot       => Neighbor::all_neighbors(),

            Ship::LeftEnd   => Neighbor::all_except(Neighbor::E),
            Ship::RightEnd  => Neighbor::all_except(Neighbor::W),
            Ship::TopEnd    => Neighbor::all_except(Neighbor::S),
            Ship::BottomEnd => Neighbor::all_except(Neighbor::N),

            Ship::VerticalMiddle => [
                Neighbor::NE, Neighbor::E, Neighbor::SE,
                Neighbor::NW, Neighbor::W, Neighbor::SW,
            ].into_iter().cloned().collect(),
            Ship::HorizontalMiddle => [
                Neighbor::NW, Neighbor::N, Neighbor::NE,
                Neighbor::SW, Neighbor::S, Neighbor::SE,
            ].into_iter().cloned().collect(),
            Ship::AnyMiddle => [
                Neighbor::NW, Neighbor::NE,
                Neighbor::SW, Neighbor::SE,
            ].into_iter().cloned().collect(),
        }
    }

}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Square {
    Unknown,
    Water,
    Ship(Ship)
}

impl Square {
    pub fn is_ship(self) -> bool {
        match self {
            Square::Ship(_) => true,
            _               => false
        }
    }

    pub fn is_ship_middle(self) -> bool {
        match self {
            Square::Ship(Ship::AnyMiddle)        |
            Square::Ship(Ship::VerticalMiddle)   |
            Square::Ship(Ship::HorizontalMiddle) => true,

            _ => false,            
        }
    }

    pub fn from_char(square_char: char) -> Option<Self> {
        match square_char {
            ' ' => Some(Square::Unknown),
            '~' => Some(Square::Water),
            '*' => Some(Square::Ship(Ship::Any)),
            '•' => Some(Square::Ship(Ship::Dot)),
            '<' => Some(Square::Ship(Ship::LeftEnd)),
            '>' => Some(Square::Ship(Ship::RightEnd)),
            '^' => Some(Square::Ship(Ship::TopEnd)),
            'v' => Some(Square::Ship(Ship::BottomEnd)),
            '|' => Some(Square::Ship(Ship::VerticalMiddle)),
            '-' => Some(Square::Ship(Ship::HorizontalMiddle)),
            '☐' => Some(Square::Ship(Ship::AnyMiddle)),
            _   => None,
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
                Ship::AnyMiddle        => '☐',
            }
        };

        return write!(f, "{}", char)
    }
}

impl From<char> for Square {
    fn from(square_char : char) -> Self {
        match Square::from_char(square_char) {
            Some(square) => square,
            None         => panic!("Unknown char".to_string()),
        }
    }
}

