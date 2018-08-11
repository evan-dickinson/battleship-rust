use square::*;
use neighbor::*;
use board::*;

mod fill_unknown;
use self::fill_unknown::*;

mod surround;
use self::surround::*;

mod specify_ships;
use self::specify_ships::*;

// TODO: Checks to implement:
// * Count ships (1 x size 3, 2 x size 2, etc.)

fn place_ships_next_to_ends(board: &mut Board, changed: &mut bool) {
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

                board.set(neighbor_coord, Square::Ship(Ship::Any), changed);
            }
        }
    }        
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

pub fn solve(board : &mut Board) {
    let solvers = [
        fill_with_water,
        fill_with_ships,
        surround_ships_with_water,
        place_ships_next_to_ends,
    ];

    board.print();
    loop {
        let mut changed_in_loop = false;

        for solve in solvers.iter() {
            let mut changed_in_step = false;
            solve(board, &mut changed_in_step);
            if changed_in_step {
                board.print();
            }

            changed_in_loop = changed_in_loop || changed_in_step;
        }

        if changed_in_loop == false {
            break;
        }
    }
}

