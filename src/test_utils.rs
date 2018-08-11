#[cfg(test)]
use board::*;

#[cfg(test)]
pub fn make_test_board() -> Board {
    let text = vec![
        "  1101",
        "1|~~* ",
        "2|  *~",
    ];

    return Board::new(text);
}
