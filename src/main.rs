mod neighbor;
mod square;
mod layout;
mod board;
mod test_utils;
mod solve;

use self::board::*;
use self::solve::*;	

fn main() {
    let _puzzle1 = vec![
        "  112121",
        "2|      ",
        "0|      ",
        "4| >    ",
        "0|      ",
        "2|     •",
        "0|      ",
    ];

    let _puzzle2 = vec![
        "  1304131",
        "0|       ",
        "5|       ",
        "0|       ",
        "1| >     ",
        "2|       ",
        "2|       ",
        "3|       ",
    ];

    let _puzzle3 = vec![
        "  3014161320",
        "0|         •",
        "4|          ",
        "1| <        ",
        "0|          ",
        "3|          ",
        "2|        v ",
        "2|v         ",
        "4|          ",
        "2|          ",
        "3|   ~      ",
    ];

    let mut board = Board::new(_puzzle3);

    solve(&mut board);

    board.print();

    if board.is_solved() {
        println!("Solved 😀");
    }
    else {
        println!("Not solved 😞");
    }
}
