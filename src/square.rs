use std::fmt;
use std::collections::HashSet;

use crate::neighbor::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ShipSquare {
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
use self::ShipSquare::*;

impl ShipSquare {
    pub fn all() -> impl Iterator<Item = Self> {
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
        ].iter().cloned()
    }

    // For a given ship type, which neigbors should be set to water
    pub fn water_neighbors(self) -> HashSet<Neighbor> {
        use crate::neighbor::Neighbor::*;
        match self {
            Any       => [
                NW, NE, SW, SE,
            ].iter().cloned().collect(),

            Dot       => Neighbor::all_neighbors(),

            LeftEnd   => Neighbor::all_except(E),
            RightEnd  => Neighbor::all_except(W),
            TopEnd    => Neighbor::all_except(S),
            BottomEnd => Neighbor::all_except(N),

            VerticalMiddle => [
                NE, E, SE,
                NW, W, SW,
            ].iter().cloned().collect(),
            HorizontalMiddle => [
                NW, N, NE,
                SW, S, SE,
            ].iter().cloned().collect(),
            AnyMiddle => [
                NW, NE,
                SW, SE,
            ].iter().cloned().collect(),
        }
    }

    // For a given ship type, which neighbors should be set to ships
    pub fn ship_neighbors(self) -> HashSet<Neighbor> {
        let all_neighbors = Neighbor::all_neighbors();
        let water_neighbors = self.water_neighbors();

        // Any squre that's not water is a ship
        // TODO: Not true for AnyMiddle!
        all_neighbors.difference(&water_neighbors).cloned().collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Square {
    Unknown,
    Water,
    ShipSquare(ShipSquare)
}
use self::Square::*;

impl Square {
    pub fn is_ship(self) -> bool {
        match self {
            ShipSquare(_) => true,
            _             => false
        }
    }

    pub fn is_ship_middle(self) -> bool {
        match self {
            ShipSquare(AnyMiddle)        |
            ShipSquare(VerticalMiddle)   |
            ShipSquare(HorizontalMiddle) => true,

            _ => false,            
        }
    }

    pub fn from_char(square_char: char) -> Option<Self> {
        match square_char {
            ' ' => Some(Unknown),
            '~' => Some(Water),
            '*' => Some(ShipSquare(Any)),
            '•' => Some(ShipSquare(Dot)),
            '<' => Some(ShipSquare(LeftEnd)),
            '>' => Some(ShipSquare(RightEnd)),
            '^' => Some(ShipSquare(TopEnd)),
            'v' => Some(ShipSquare(BottomEnd)),
            '|' => Some(ShipSquare(VerticalMiddle)),
            '-' => Some(ShipSquare(HorizontalMiddle)),
            '☐' => Some(ShipSquare(AnyMiddle)),
            _   => None,
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char = match self {
            Unknown => ' ',
            Water   => '~',

            ShipSquare(ship_type) => match ship_type {
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

