use std::fmt;

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
    pub fn is_ship(&self) -> bool {
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

