mod neighbor;
mod square;
mod layout;
mod board;
mod test_utils;
mod solve;

use self::board::*;
use self::solve::*;	

fn main() {
    let mut board = Board::new(vec![
        "  112121",
        "2|      ",
        "0|      ",
        "4| >    ",
        "0|      ",
        "2|     •",
        "0|      ",
    ]);

    solve(&mut board);

    board.print();
}
