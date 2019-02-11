// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod board;
mod error;
mod layout;
mod neighbor;
mod parse;
mod ship;
mod solve;
mod square;
mod test_utils;

use crate::board::*;
use crate::solve::*;	
use crate::error::*;

fn run() -> Result<()> {
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

    // https://lukerissacher.com/battleships/PQhwIQIJJC-CEhAHAFMAI-AEAAcBFdAO-AAQABIAAAA-AI4AOQ
    let _puzzle5 = vec![
        "ships: 5sq x 1, 4sq x 2, 3sq x 3, ",
        "       2sq x 4, 1sq x 4.",    
        "  150405130033020",
        "2|         •     ",
        "3|               ",
        "5|               ",
        "4|               ",
        "0|     v         ",
        "3|   ~           ",
        "1|               ",
        "0|          ☐    ",
        "4|        >      ",
        "0|   ☐           ",
        "1|               ",
        "2|               ",
        "0|               ",
        "1|             ^ ",
        "1|          v    ",
    ];

    let _puzzle6 = vec![
        "ships: 4sq x 1, 3sq x 1, ",
        "       2sq x 2, 1sq x 3.",
        "  3141401",
        "2|  ~    ",
        "1|       ",
        "4|       ",
        "0|       ",
        "1|       ",
        "3|       ",
        "3|       ",
    ];

    let _puzzle7 = vec![
        "ships: 4sq x 1, 3sq x 1, ",
        "       2sq x 2, 1sq x 3.",
        "  1420213",
        "2|       ",
        "2|   <   ",
        "1|       ",
        "1|       ",
        "3|       ",
        "1|       ",
        "3|       ",
    ];

    // https://lukerissacher.com/battleships/ICdJJIEkDo-Q4A4kg
    let _puzzle8 = vec![
        "ships: 4sq x 1, 3sq x 2, ",
        "       2sq x 3, 1sq x 3.",
        "  40405020",
        "1|      ^ ",
        "4|        ",
        "2|        ",
        "3|        ",
        "1|  v     ",
        "1|    v   ",
        "0|      ^ ",
        "3|        ",
    ];

    let mut board = Board::new(&_puzzle8)?;

    let is_solved = solve(&mut board)?;

    board.print();

    if is_solved {
        println!("Solved 😀");
    }
    else {
        println!("Not solved 😞");
    }

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}
