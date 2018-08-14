use square::*;
use board::*;

use std;

pub fn parse_board(text: &str) -> Board {
	return match board(text) {
		Ok(("", board)) => board,
		Ok((_,  _))     => panic!("Had leftover information"),
		Err(_)          => panic!("Unable to parse the board"),
	}
}

/////////////////////////////////////////////////////////////////////
//
// Parse the column headers

named!(header_items<&str, Vec<usize>>,
	fold_many1!(ships_remaining,
		Vec::new(),
		|mut acc: Vec<usize>, item| {
			acc.push(item);
			acc
		}
	)	
);

named!(header<&str, Vec<usize>>,
	do_parse!(
		tag!("  ")           >>
		counts: header_items >>
		tag!("\n")           >>
		(counts)
	)
);

/////////////////////////////////////////////////////////////////////
//
// Parse the body of the board

#[derive(Debug)]
struct Row {
	ships_remaining: usize,
	squares: Vec<Square>,
}


fn square_from_char(input: &str) -> Result<Square, u8> {
	let c = input.chars().next().unwrap();

	return match Square::from_char(c) {
		Some(square) => Ok(square),
		None		 => Err(0),
	}
}

fn is_digit(c: char) -> bool {
  match c {
    '0'..='9' => true,
    _ => false,
  }
}

fn int_from_digit(input: &str) -> Result<usize, std::num::ParseIntError> {
  usize::from_str_radix(input, 10)
}

named!(ships_remaining<&str, usize>,
	map_res!(take_while_m_n!(1, 1, is_digit), int_from_digit)
);

named!(square<&str, Square>, 
	map_res!(take_s!(1), square_from_char)
);

named!(squares<&str, Vec<Square>>, 
	fold_many1!(square,
		Vec::new(),
		|mut acc: Vec<Square>, item| {
			acc.push(item);
			acc
		})
);

named!(row<&str, Row>,
	do_parse!(
		ships_remaining: ships_remaining >>
		                 tag!("|")       >>
		squares:         squares         >>
		                 tag!("\n")      >>
        (Row { ships_remaining, squares })
	)
);

named!(rows<&str, Vec<Row>>,
	fold_many1!(row,
		Vec::new(),
		|mut acc: Vec<Row>, item| {
			acc.push(item); acc
		}
	)
);


/////////////////////////////////////////////////////////////////////
//
// Parse the entire board

named!(board<&str, Board>,
	do_parse!(
		ships_remaining_for_col: header >>
		rows:                    rows   >>
		                         tag!(".") >>
		(make_board(ships_remaining_for_col, rows))
	)
);

fn make_board(ships_remaining_for_col: Vec<usize>, rows: Vec<Row>) -> Board {
	// Ensure all rows have the same number of cols
	let num_cols = ships_remaining_for_col.len();

	let consistent_num_cols = rows.iter().all(|row| {
		row.squares.len() == num_cols
	});
	if !consistent_num_cols {
		panic!("Rows have inconsistent number of columns");
	}

	let ships_remaining_for_row = rows.iter()
		.map(|row| row.ships_remaining)
		.collect::<Vec<_>>();

	let squares = rows.iter()
		.map(|row| row.squares.clone())
		.collect::<Vec<_>>();

	return Board::new_from_data(
		squares,
		ships_remaining_for_row,
		ships_remaining_for_col
	);
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn it_parses_header() {
    	let text = "  12345\n";
    	let result = header(text);
		
    	if let Ok((_, counts)) = result {
    		assert_eq!(counts[0], 1);
    		assert_eq!(counts[1], 2);
    		assert_eq!(counts[2], 3);
    		assert_eq!(counts[3], 4);
    		assert_eq!(counts[4], 5);
    		assert_eq!(5, counts.len());
    	}
    	else {
    		println!("{:?}", result);
    		assert!(false);
    	}
   	}

    #[test]
    fn it_parses_ships_remaining() {
    	let text = "5";
    	let result = ships_remaining(text);
		
		assert_eq!(result, Ok(("", 5)));
    }

    #[test]
    fn it_parses_one_square() {
    	let text = "~";
    	let result = square(text);
		
		assert_eq!(result, Ok(("", Square::Water)));    	
    }

    #[test]
    fn it_parses_squares() {
    	let text = "~~ \n"; // need a token that squares() doesn't recognize, so it knows when to stop
    	let result = squares(text);

    	if let Ok((_, ships)) = result {
    		assert_eq!(Square::Water, ships[0]);
    		assert_eq!(Square::Water, ships[1]);
    		assert_eq!(Square::Unknown, ships[2]);
    		assert_eq!(3, ships.len());
    	}
    	else {
    		println!("{:?}", result);
    		assert!(false);
    	}
    }

    #[test]
    fn it_parses_row() {
		let text = "1|~ ~\n";
		let result = row(text);

    	if let Ok(("", row)) = result {
    		assert_eq!(row.ships_remaining, 1);

    		assert_eq!(row.squares.len(), 3);
    		assert_eq!(row.squares[0], Square::Water);
    		assert_eq!(row.squares[1], Square::Unknown);
    		assert_eq!(row.squares[2], Square::Water);
    	}
    	else {
    		println!("{:?}", result);
    		assert!(false);
    	}
    }

    #[test]
    fn it_parses_board() {
    	let text = [
    		"  123",
    		"1|~  ",
    		"2|  *",
    		"."
    	].join("\n");

    	let board = parse_board(&text);

    	assert_eq!(board.layout.num_cols, 3);
    	assert_eq!(board.layout.num_rows, 2);
    }
}

