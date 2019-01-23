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
use self::Ship::*;

impl Ship {
    pub fn all() -> impl Iterator<Item = Ship> {
        [
            Any,
            LeftEnd,
            RightEnd,
            TopEnd,
            BottomEnd,
            VerticalMiddle,
            HorizontalMiddle,
            AnyMiddle,
            Dot
        ].into_iter().cloned()
    }

    // Return the nth square for a ship, along the given axis.
    // For example, a ship of size 3 on horizontal axis, we expect to see LeftEnd, then HorizontalMiddle, then RightEnd
    pub fn expected_square_for_ship(ship_size: usize, square_idx: usize, incrementing_axis: Axis) -> Ship {
        use crate::layout::Axis::*;

        assert!(square_idx < ship_size);

        if ship_size == 1 {
            Dot
        }
        else {
            enum Position { Start, Middle, End };
            let pos = if square_idx == 0             { Position::Start  }
                else  if square_idx == ship_size - 1 { Position::End    }
                else                                 { Position::Middle };

            match (pos, incrementing_axis) {
                (Position::Start,  Col) => LeftEnd,
                (Position::Start,  Row) => TopEnd,
                (Position::Middle, Col) => HorizontalMiddle,
                (Position::Middle, Row) => VerticalMiddle,
                (Position::End,    Col) => RightEnd,
                (Position::End,    Row) => BottomEnd,
            }
        }
    }    

    // For a given ship type, which neigbors should be set to water
    pub fn water_neighbors(self) -> HashSet<Neighbor> {
        use crate::neighbor::Neighbor::*;
        match self {
            Any       => [
                NW, NE, SW, SE,
            ].into_iter().cloned().collect(),

            Dot       => Neighbor::all_neighbors(),

            LeftEnd   => Neighbor::all_except(E),
            RightEnd  => Neighbor::all_except(W),
            TopEnd    => Neighbor::all_except(S),
            BottomEnd => Neighbor::all_except(N),

            VerticalMiddle => [
                NE, E, SE,
                NW, W, SW,
            ].into_iter().cloned().collect(),
            HorizontalMiddle => [
                NW, N, NE,
                SW, S, SE,
            ].into_iter().cloned().collect(),
            AnyMiddle => [
                NW, NE,
                SW, SE,
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
use self::Square::*;

impl Square {
    pub fn is_ship(self) -> bool {
        match self {
            Ship(_) => true,
            _       => false
        }
    }

    pub fn is_ship_middle(self) -> bool {
        match self {
            Ship(AnyMiddle)        |
            Ship(VerticalMiddle)   |
            Ship(HorizontalMiddle) => true,

            _ => false,            
        }
    }

    pub fn from_char(square_char: char) -> Option<Self> {
        match square_char {
            ' ' => Some(Unknown),
            '~' => Some(Water),
            '*' => Some(Ship(Any)),
            '•' => Some(Ship(Dot)),
            '<' => Some(Ship(LeftEnd)),
            '>' => Some(Ship(RightEnd)),
            '^' => Some(Ship(TopEnd)),
            'v' => Some(Ship(BottomEnd)),
            '|' => Some(Ship(VerticalMiddle)),
            '-' => Some(Ship(HorizontalMiddle)),
            '☐' => Some(Ship(AnyMiddle)),
            _   => None,
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char = match self {
            Unknown => ' ',
            Water   => '~',

            Ship(ship_type) => match ship_type {
                Any              => '*',
                Dot              => '•',              
                LeftEnd          => '<',
                RightEnd         => '>',
                TopEnd           => '^',
                BottomEnd        => 'v',
                VerticalMiddle   => '|',
                HorizontalMiddle => '-',
                AnyMiddle        => '☐',
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

