// rustc --crate-type lib --emit llvm-ir lib.rs -O

// use std::collections::HashSet;
// use std::iter::FromIterator;
// use std::iter::IntoIterator;

mod neighbor;
use self::neighbor::*;

mod square;
use self::square::*;

mod layout;
use self::layout::*;

mod board;
use self::board::*;

mod test_utils;
use self::test_utils::*;

mod solve;
use self::solve::*;





