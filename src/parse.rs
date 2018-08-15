use std::collections::HashMap;

use square::*;
use board::*;

use std;

/////////////////////////////////////////////////////////////////////
//
// Utility functions

fn is_digit(c: char) -> bool {
  match c {
    '0'..='9' => true,
    _ => false,
  }
}

fn int_from_digit(input: &str) -> Result<usize, std::num::ParseIntError> {
  usize::from_str_radix(input, 10)
}

/////////////////////////////////////////////////////////////////////
//
// Parse ships to find

#[derive(Debug)]
struct ShipToFind {
	size: usize,
	count: usize,
}

named!(ship_to_find<&str, ShipToFind>,
	do_parse!(
		size: map_res!(take_while_m_n!(1, 1, is_digit), int_from_digit) >>
			ws!(
				tuple!(
		      		tag!("sq"),
		      		tag!("x")
		      	)
		    ) >>
		count: map_res!(take_while_m_n!(1, 1, is_digit), int_from_digit) >>
		(ShipToFind { size, count })
	)
);

named!(ships_to_find<&str, Vec<ShipToFind>>,
	do_parse!(
		tag!("ships:") >>
		ships: ws!(separated_nonempty_list_complete!(
				ws!(tag!(",")),
				ship_to_find
		)) >>
		tag!(".\n") >>
		(ships)
	)
);


#[cfg(test)]
mod test_ships_to_find {
    use super::*;

    #[test]
    fn it_parses_individual_ship() {
    	let text = "4sq x 2";

    	let result = ship_to_find(&text);

    	if let Ok(("", ship)) = result {
    		assert_eq!(ship.size, 4);
    		assert_eq!(ship.count, 2);    		
    	}
    	else {
    		println!("{:?}", result);
    		assert!(false);
    	}    	
    }

    #[test]
    fn it_parses_1_ship() {
    	let text = "ships: 5sq x 1.\n";

    	let result = ships_to_find(&text);

    	if let Ok(("", ships)) = result {
	    	assert_eq!(ships.len(), 1);

	    	assert_eq!(ships[0].size, 5);
	    	assert_eq!(ships[0].count, 1);
    	}
    	else {
    		println!("{:?}", result);
    		assert!(false);
    	}  
    }

    #[test]
    fn it_parses_2_ships() {
    	let text = "ships: 5sq x 1, 4sq x 2.\n";

    	let result = ships_to_find(&text);

    	if let Ok(("", ships)) = result {
	    	assert_eq!(ships.len(), 2);

	    	assert_eq!(ships[0].size, 5);
	    	assert_eq!(ships[0].count, 1);

	    	assert_eq!(ships[1].size, 4);
	    	assert_eq!(ships[1].count, 2);
    	}
    	else {
    		println!("{:?}", result);
    		assert!(false);
    	}  
    }

    #[test]
    fn it_parses_5_ships() {
        let text = "ships: 5sq x 1, 4sq x 1, 3sq x 2, 2sq x 3, 1sq x 4.\n";

        let result = ships_to_find(&text);

        if let Ok(("", ships)) = result {
            assert_eq!(ships.len(), 5);

            assert_eq!(ships[0].size, 5);
            assert_eq!(ships[0].count, 1);

            assert_eq!(ships[1].size, 4);
            assert_eq!(ships[1].count, 1);

            assert_eq!(ships[2].size, 3);
            assert_eq!(ships[2].count, 2);   

            assert_eq!(ships[3].size, 2);
            assert_eq!(ships[3].count, 3);   

            assert_eq!(ships[4].size, 1);
            assert_eq!(ships[4].count, 4);            
        }
        else {
            println!("{:?}", result);
            assert!(false);
        }  
    }    

    #[test]
    fn it_parses_ships_on_multiple_lines() {
    	let text = "ships: 5sq x 1,\n\t4sq x 2.\n";

    	let result = ships_to_find(&text);

    	if let Ok(("", ships)) = result {
	    	assert_eq!(ships.len(), 2);

	    	assert_eq!(ships[0].size, 5);
	    	assert_eq!(ships[0].count, 1);

	    	assert_eq!(ships[1].size, 4);
	    	assert_eq!(ships[1].count, 2);
    	}
    	else {
    		println!("{:?}", result);
    		assert!(false);
    	}  
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


#[cfg(test)]
mod column_header_tests {
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
}   	

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

#[cfg(test)]
mod board_body_tests {
    use super::*;

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
}


/////////////////////////////////////////////////////////////////////
//
// Parse the entire board

named!(board<&str, Board>,
	do_parse!(
		ships_to_find:           opt!(ships_to_find) >>
		ships_remaining_for_col: header >>
		rows:                    rows   >>
		                         tag!(".") >>
		(make_board(ships_to_find, ships_remaining_for_col, rows))
	)
);

fn make_board(ships_to_find_vec: Option<Vec<ShipToFind>>, 
	ships_remaining_for_col: 
	Vec<usize>, rows: Vec<Row>) -> Board {

	// Convert ships_to_find from vector to hash map
	let ships_to_find;
	if let Some(vec) = ships_to_find_vec {
		ships_to_find = vec.iter()
			.fold(HashMap::new(), |mut acc, ship| {
				acc.insert(ship.size, ship.count);
				acc
			});
	}
	else {
		ships_to_find = HashMap::new();
	}

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
		ships_remaining_for_col,
		ships_to_find
	);
}

pub fn parse_board(text: &str) -> Board {
	return match board(text) {
		Ok(("", board)) => board,
		Ok((_,  _))     => panic!("Had leftover information"),
		Err(_)          => panic!("Unable to parse the board"),
	}
}

#[cfg(test)]
mod board_tests {
    use super::*;
    use layout::*;

    #[test]
    fn it_parses_board_no_ships_to_find() {
    	let text = [
    		"  123",
    		"1|~  ",
    		"2|  *",
    		"."
    	].join("\n");

    	let board = parse_board(&text);

    	assert_eq!(board.layout.num_cols, 3);
    	assert_eq!(board.layout.num_rows, 2);

    	assert_eq!(board[Coord{row_num: 1, col_num: 2}], Square::Ship(Ship::Any));
    }

    #[test]
    fn it_parses_board_with_ships_to_find() {
    	let text = [
    		"ships: 2sq x 1, 1sq x 3.",
    		"  123",
    		"1|~  ",
    		"2|  *",
    		"."
    	].join("\n");

    	let board = parse_board(&text);

    	assert_eq!(board.layout.num_cols, 3);
    	assert_eq!(board.layout.num_rows, 2);

    	assert_eq!(board[Coord{row_num: 1, col_num: 2}], Square::Ship(Ship::Any));

    	assert_eq!(board.ships_to_find_for_size(3), 0);
    	assert_eq!(board.ships_to_find_for_size(2), 1);
    	assert_eq!(board.ships_to_find_for_size(1), 3);
    }    
}

