// rustc --crate-type lib --emit llvm-ir lib.rs -O

pub mod client;
pub mod network;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Square {
	Unknown,
	Water,
	Ship
}

pub fn char_to_square(square_char : char) -> Result<Square, String> {
	match square_char {
		' ' => Ok(Square::Unknown),
		'*' => Ok(Square::Ship),
		'~' => Ok(Square::Water),
		_   => Err("Unknown char".to_string())
	}
}

pub fn str_to_row(line : &str) -> Result<Vec<Square>, String> {
	let row : Result<Vec<Square>, String> = line.chars()
		.map(char_to_square)
		.collect();

	return row;	
}

pub fn str_to_rows(text : &str) -> Result<Vec<Vec<Square>>, String> {
	let rows : Result<Vec<Vec<Square>>, String> = text.split("\n")
		.map(str_to_row)
		.collect();

	return rows;		
}


#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn it_finds_a_ship() {
    	let result = char_to_square('*');

        assert_eq!(result, Ok(Square::Ship));
    }

    #[test]
    fn it_fails() {
    	let result = char_to_square('q');

        assert_eq!(result, Err("Unknown char".to_string()));
    }

    #[test]
    fn it_makes_a_row() {
    	let line = "~~* ";
    	let row = str_to_row(&line);

    	let expected_row = Ok(vec![
    		Square::Water,
    		Square::Water,
    		Square::Ship,
    		Square::Unknown
    	]);

    	assert_eq!(row, expected_row);
    }

    #[test]
    fn it_fails_to_make_a_row() {
    	let line = "~q~";
    	let row = str_to_row(&line);

    	let expected_row = Err("Unknown char".to_string());

    	assert_eq!(row, expected_row);
    }    

    #[test]
    fn it_makes_several_rows() {
    	let text = "~~* \n  *~";
    	let rows = str_to_rows(text);

    	let expected = Ok(vec![
    		vec![
    			Square::Water,
    			Square::Water,
    			Square::Ship,
    			Square::Unknown,    			
    		],
    		vec![
    			Square::Unknown,
    			Square::Unknown,
    			Square::Ship,
    			Square::Water
    		],
    	]);

    	assert_eq!(rows, expected);
    }
}
