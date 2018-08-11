use square::*;
use board::*;
use neighbor::*;
use layout::*;

pub fn place_ships_next_to_ends(board: &mut Board, changed: &mut bool) {
    let layout = board.layout;
    let ship_coords = {
        layout.all_coordinates()
            .filter(|coord| { board[*coord].is_ship() })
            .collect::<Vec<_>>()
    };
    for coord in ship_coords {
        let neighbor = match board[coord] {
            Square::Ship(Ship::TopEnd) => Some(Neighbor::S),
            Square::Ship(Ship::BottomEnd) => Some(Neighbor::N),
            Square::Ship(Ship::LeftEnd) => Some(Neighbor::E),
            Square::Ship(Ship::RightEnd) => Some(Neighbor::W),
            _ => None,
        };

        if let Some(neighbor) = neighbor {
            if let Some(neighbor_coord) = 
                layout.coord_for_neighbor(coord, neighbor)  {


                if board[neighbor_coord] == Square::Unknown {
                	board.set(neighbor_coord, Square::Ship(Ship::Any), changed);
                }
            }
        }
    }        
}

fn do_test(before: Vec<&str>, after: Vec<&str>) {
	let mut board = Board::new(before);
	let expected = after.iter().map(|x| x.to_string()).collect::<Vec<_>>();

    let mut _changed = false;
    place_ships_next_to_ends(&mut board, &mut _changed);
    assert_eq!(board.to_strings(), expected);        
}

#[test]
fn it_places_ships_next_to_ends() {
    let mut board = Board::new(vec![
        "  00100",
        "0|  ^  ",
        "1|     ",
    ]);

    let mut _changed = false;
    place_ships_next_to_ends(&mut board, &mut _changed);
    let expected = vec![
        "  00000",
        "0|  ^  ",
        "0|  *  ",    
    ].iter().map(|x| x.to_string()).collect::<Vec<_>>();
    assert_eq!(board.to_strings(), expected);
}

#[test]
fn it_doesnt_overwrite_existing_ship() {
	// If it tries to overwrite v because it's adjacent to ^, board.set will assert.
	// Make sure that doesn't happen.
	do_test(vec![
        "  00100",
        "0|  ^  ",
        "1|  v  ",	
    ],
	vec![
        "  00100",	
        "0|  ^  ",
        "1|  v  ",	
   	]);
}
