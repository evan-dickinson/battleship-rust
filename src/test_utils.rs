#![cfg(test)]

use crate::board::*;
use crate::error::*;

pub fn make_test_board() -> Result<Board> {
    let text = vec![
        "  1101",
        "1|~~* ",
        "2|  *~",
    ];

    Board::new(&text)
}
