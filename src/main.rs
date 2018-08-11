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
        "  122122",
        "2|      ",
        "0|      ",
        "5| >    ",
        "0|      ",
        "3|     •",
        "0|      ",
    ]);

    solve(&mut board);

    for str in board.to_strings() {
    	println!("{}", str);
    }
}
