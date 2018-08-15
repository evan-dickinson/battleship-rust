use std::fmt;

use layout::*;

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

impl Ship {
    pub fn all() -> Vec<Ship> {
        return vec! [
            Ship::Any,
            Ship::LeftEnd,
            Ship::RightEnd,
            Ship::TopEnd,
            Ship::BottomEnd,
            Ship::VerticalMiddle,
            Ship::HorizontalMiddle,
            Ship::Dot
        ];
    }

    // Return the nth square for a ship, along the given axis.
    // For example, a ship of size 3 on horizontal axis, we expect to see LeftEnd, then HorizontalMiddle, then RightEnd
    pub fn expected_square_for_ship(ship_size: usize, square_idx: usize, incrementing_axis: Axis) -> Ship {
        assert!(square_idx < ship_size);

        if ship_size == 1 {
            return Ship::Dot;
        }
        else {
            if square_idx == 0 {
                return match incrementing_axis {
                    // If we're incrementing columns, need to start with a left end.
                    // If incrementing rows, need to start with a top end.
                    Axis::Col => Ship::LeftEnd,
                    Axis::Row => Ship::TopEnd,
                }
            }
            else if square_idx == ship_size - 1 {
                return match incrementing_axis {
                    Axis::Col => Ship::RightEnd,
                    Axis::Row => Ship::BottomEnd,
                }           
            }
            else { // middle
                return match incrementing_axis {
                    Axis::Col => Ship::HorizontalMiddle,
                    Axis::Row => Ship::VerticalMiddle,
                }
            }
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
    pub fn is_ship(&self) -> bool {
        match self {
            Square::Ship(_) => true,
            _               => false
        }
    }

    pub fn from_char(square_char : char) -> Option<Self> {
        return match square_char {
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
            }
        };

        return write!(f, "{}", char)
    }
}

impl From<char> for Square {
    fn from(square_char : char) -> Self {
        return match Square::from_char(square_char) {
            Some(square) => square,
            None         => panic!("Unknown char".to_string()),
        }
    }
}

