#[macro_use]
extern crate nom;

mod neighbor;
mod square;
mod layout;
mod board;
mod test_utils;
mod solve;
mod parse;

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
        "ships: 5sq x 1, 4sq x 1, 3sq x 2, ",
        "       2sq x 3, 1sq x 4.",
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

    let _puzzle4 = vec![
        // May not be deterministically solvable
        "ships: 5sq x 1, 4sq x 2, 3sq x 3, ",
        "       2sq x 4, 1sq x 4.",
        "  021343411141121",
        "0|       •       ",
        "3|               ",
        "1|          ☐    ",
        "0|               ",
        "1|               ",
        "1|               ",
        "4|     v     ☐   ",
        "1|               ",
        "5|               ",
        "0|          v    ",
        "5|               ",
        "4|        •      ",
        "3|               ",
        "0|               ",
        "0|               ",
    ];    

    let mut board = Board::new(_puzzle4);

    solve(&mut board);

    board.print();

    if board.is_solved() {
        println!("Solved 😀");
    }
    else {
        println!("Not solved 😞");
    }
}
