/////////////////////////////////////////////////////////////////////
//
// Solutions for the "only place it can go" rule

use crate::square::*;
use crate::board::*;
use crate::layout::*;

use std::collections::HashSet;
use std::hash::Hash;

use smallvec::SmallVec;

pub fn find_only_place_for_ships(board: &mut Board, changed : &mut bool) {
    let sizes = board.remaining_ship_sizes().collect::<SmallVec<[_; 6]>>();

    for ship_size in sizes {
        let num_ships = board.ships_to_find_for_size(ship_size);

        find_only_place_for_ship(board, ship_size, num_ships, changed);
    }
}

fn find_only_place_for_ship(board: &mut Board, ship_size: usize, num_ships: usize, changed: &mut bool) {
    let layout = board.layout;
    let placements = layout.possible_coords_for_ship(ship_size)
        .filter_map(|(coord, incrementing_axis)| {
            let constant_axis = incrementing_axis.cross_axis();

            if let Some(num_ship_squares) = can_fit_ship_at_coord(board, ship_size, coord, incrementing_axis) {
                if would_ship_at_coord_be_clear_of_other_ships(board, ship_size, coord, incrementing_axis) &&  
                   enough_free_ships_on_constant_axis(board, ship_size, coord, constant_axis, num_ship_squares) &&
                   enough_free_ships_on_incrementing_axis(board, ship_size, coord, incrementing_axis) {
                    Some((coord, incrementing_axis))
                }
                else {
                    None
                }
            }
            else {
                None
            }
        })
        .collect::<Vec<_>>();

    if placements.len() == num_ships {
        // We know the placement of every square in the ships. Fill in the complete ships.

        for (coord, incrementing_axis) in placements {
            place_ship_at_coord(board, ship_size, coord, incrementing_axis, changed);
        }
    }
    else {
        // We can't fill in *all* the squares of the ships, but perhaps we can fill in *some*.
        // Example: We need to fill a ship of size 4 and there are 5 possible contiguous squares. We know
        // the middle 3 squares will be ships, even if we don't know which square will hold the 4th
        // ship.
        //
        // Also handle cases where there are two ships that need to be placed, and also two gaps where
        // ships can go.

        // For each placement, create a HashSet of coordinates in that placement
        let coordinates_for_all_placements = placements.iter().map(|(coord, incrementing_axis)| {
            // Collect coordinates for the current placement
            (0..ship_size)
                .map(|square_idx| coord.offset(square_idx, *incrementing_axis).unwrap() )
                .collect::<HashSet<_>>()
        }).collect::<Vec<_>>();

        let partitioned_coordinates = partition(coordinates_for_all_placements);

        if partitioned_coordinates.len() == num_ships {
            place_ship_at_intersection_of_coords(board, &mut partitioned_coordinates.into_iter(), changed);
        }
    }
}

// After determining we can place a ship here, place it.
fn place_ship_at_coord(board: &mut Board, ship_size: usize, coord: Coord, incrementing_axis: Axis, changed: &mut bool) {
    for square_idx in 0..ship_size {
        let coord = coord.offset(square_idx, incrementing_axis).unwrap();
        let ship_square_type = ShipSquare::expected_square_for_ship(ship_size, square_idx, incrementing_axis);
        let new_value = Square::ShipSquare(ship_square_type);
        board.set(coord, new_value, changed);
    }
}

fn place_ship_at_intersection_of_coords<'a>(board: &mut Board, coordinates_for_all_placements: &mut impl Iterator<Item = HashSet<Coord<'a>>>, 
    changed: &mut bool) {

    for coords in coordinates_for_all_placements {
        // Set coordinates in the intersection
        for coord in coords {
            if board[coord] == Square::Unknown {
                board.set(coord, Square::ShipSquare(ShipSquare::Any), changed);
            }
        }        
    }
}

// constant axis: The one that remains the same as we increment through coordinats
// incrementing axis: The one that changes as we increment through coordinates
fn enough_free_ships_on_constant_axis(board: &Board, ship_size: usize, coord: Coord, constant_axis: Axis, num_ship_squares: usize) -> bool {
    let row_or_col = coord.row_or_col(constant_axis);
    let ships_remaining = board.ships_remaining(row_or_col);

    ships_remaining >= ship_size - num_ship_squares
}

// In the incrementing axis, need to have one ship remaining per square
fn enough_free_ships_on_incrementing_axis(board: &Board, ship_size: usize, coord: Coord, incrementing_axis: Axis) -> bool {
    for square_idx in 0..ship_size {
        if let Some(coord) = coord.offset(square_idx, incrementing_axis) {
            let row_or_col = coord.row_or_col(incrementing_axis);
            let ships_remaining = board.ships_remaining(row_or_col);
            if ships_remaining < 1 && !board[coord].is_ship() {
                return false;
            }
        }
        else {
            return false;
        }
    }

    true
}

// Will the ship fit on the board at the given coordinates?
//
// Return:
// Some(usize) -- Ship fits. Usize = number of ship squares already placed 
// None        -- One of the following:
//                - Ship does not fit here
//                - Ship is already placed here
fn can_fit_ship_at_coord(board: &Board, ship_size: usize, origin: Coord, incrementing_axis: Axis) -> Option<usize> {
    let mut num_ship_squares = 0;
    let mut all_matches_exact = true;

    let fits = board.layout.coords_in_ship(ship_size, origin, incrementing_axis)
        .enumerate()
        .all(|(square_idx, curr_coord)| {
            if board[curr_coord].is_ship() {
                num_ship_squares += 1;
            }

            let expected = ShipSquare::expected_square_for_ship(ship_size, square_idx, incrementing_axis);
            
            let is_exact_match =  board[curr_coord] == Square::ShipSquare(expected);
            let is_match = is_exact_match ||
                board[curr_coord] == Square::Unknown || 
                board[curr_coord] == Square::ShipSquare(ShipSquare::Any) ||
                (
                    board[curr_coord] == Square::ShipSquare(ShipSquare::AnyMiddle) &&
                    (expected == ShipSquare::VerticalMiddle || expected == ShipSquare::HorizontalMiddle)
                );              

            all_matches_exact = all_matches_exact && is_exact_match;

            is_match
        });

    if fits && !all_matches_exact {
        Some(num_ship_squares)
    }
    else {
        None
    }
}

// Would placing a ship here cause it to touch another ship?
// Return: FALSE if it would touch another ship, TRUE if it would not touch.
fn would_ship_at_coord_be_clear_of_other_ships(board: &Board, ship_size: usize, origin: Coord, incrementing_axis: Axis) -> bool {
    board.layout.coords_in_ship(ship_size, origin, incrementing_axis)
    .enumerate()
    .all(|(square_idx, curr_coord)| {
        let is_first_square  = square_idx == 0;
        let is_last_square   = square_idx == ship_size - 1;
        let is_middle_square = !is_first_square && !is_last_square;
        // Note: If this square is a dot, it can be both a first and last square
        assert!(is_first_square || is_middle_square || is_last_square);

        let (first_square_neighbors, middle_square_neighbors, last_square_neighbors) =
            match incrementing_axis {
                Axis::Col => ( // Horizonal ship increments by columns
                    ShipSquare::LeftEnd.water_neighbors(),
                    ShipSquare::HorizontalMiddle.water_neighbors(),
                    ShipSquare::RightEnd.water_neighbors()
                ),
                Axis::Row => ( // Vertical ship increments by rows
                    ShipSquare::TopEnd.water_neighbors(),
                    ShipSquare::VerticalMiddle.water_neighbors(),
                    ShipSquare::BottomEnd.water_neighbors()
                )
            };

        // Find the neighbors that are relevant to check for curr_coord
        [
            (is_first_square,  first_square_neighbors),
            (is_middle_square, middle_square_neighbors),
            (is_last_square,   last_square_neighbors),
        ].into_iter()
        .filter_map(|(is_used, neighbors)|
            // is_used means "do we want to check these neighbors"
            if *is_used { Some(neighbors) } else { None }
        )
        .flatten()
        .filter_map(|&neighbor| curr_coord.neighbor(neighbor))
        .all(|neighbor_coord| !board[neighbor_coord].is_ship() )
    })
}


// Input:  Sets of coordinates
// Output: The input coordinates, grouped into sets of sets. Sets of coordinates that have
//         coordinates in common (i.e., non-empty intersections) are grouped together into
//         sets of sets.
// TODO: Can this be rewritten to take impl Iterator instead?
fn partition<T: Eq + Hash + Clone>(unpartitioned: Vec<HashSet<T>>) -> Vec<HashSet<T>> {
    let mut partitioned = Vec::new();
    let mut unpartitioned_iter = unpartitioned.into_iter();

    while let Some(first_set) = unpartitioned_iter.next() {
        let (mut intersecting_sets, other_sets) : (Vec<HashSet<T>>, Vec<HashSet<T>>) =
            unpartitioned_iter.partition(|curr_set|
                // .next() == None when intersection is empty (no coords in common)
                first_set.intersection(curr_set).next() != None
            );

        intersecting_sets.push(first_set);
        let intersection : HashSet<T> = common_coordinates(&mut intersecting_sets.into_iter()).unwrap(); // shouldn't fail b/c iterator is non-empty
        partitioned.push(intersection);

        unpartitioned_iter = other_sets.into_iter();
    }

    partitioned
}

// Returns an intersection of coordinates in all the sets. Or returns None if the iterator is empty.
//
// TODO: Try to use this with the ? early return
fn common_coordinates<T : Eq + Hash + Clone>(all_sets_of_coordinates: &mut impl Iterator<Item = HashSet<T>>) -> Option<HashSet<T>> {
    match all_sets_of_coordinates.next() {
        Some(first_set_of_coordinates) => {
            let common_coordinates = all_sets_of_coordinates.fold(first_set_of_coordinates, |acc, curr_set_of_coordinates| {
                acc.intersection(&curr_set_of_coordinates).cloned().collect::<HashSet<_>>()
            });
            Some(common_coordinates)
        }
        None => None
    }
}

#[cfg(test)] 
mod test_partition {
    use super::*;

    #[test]
    fn it_intersects_two_overlapping_sets() {
        let sets = vec![
            vec![1, 2, 3],
            vec![2, 3, 4],
        ].into_iter()
        .map(|vec| vec.into_iter().collect::<HashSet<_>>() )
        .collect::<Vec<HashSet<_>>>();

        let expected = vec![
            vec![2, 3],
        ].into_iter()
        .map(|vec| vec.into_iter().collect::<HashSet<_>>() )
        .collect::<Vec<HashSet<_>>>();

        let actual = partition(sets);
        assert_eq!(actual, expected);
    }    

    #[test]
    fn it_does_not_intersect_non_overlapping_sets() {
        let sets = vec![
            vec![1, 2, 3],
            vec![7, 8, 9],
        ].into_iter()
        .map(|vec| vec.into_iter().collect::<HashSet<_>>() )
        .collect::<Vec<HashSet<_>>>();

        let expected = vec![
            vec![1, 2, 3],
            vec![7, 8, 9],        
        ].into_iter()
        .map(|vec| vec.into_iter().collect::<HashSet<_>>() )
        .collect::<Vec<HashSet<_>>>();

        let actual = partition(sets);
        assert_eq!(actual, expected);
    }    

    #[test]
    fn it_supports_multiple_regions_of_overlap() {
        let sets = vec![
            vec![1, 2, 3],
            vec![2, 3, 4],

            vec![7, 8, 9],
            vec![6, 7, 8],
        ].into_iter()
        .map(|vec| vec.into_iter().collect::<HashSet<_>>() )
        .collect::<Vec<HashSet<_>>>();

        let expected = vec![
            vec![2, 3],
            vec![7, 8],
        ].into_iter()
        .map(|vec| vec.into_iter().collect::<HashSet<_>>() )
        .collect::<Vec<HashSet<_>>>();

        let actual = partition(sets);
        assert_eq!(actual, expected);
    }    
}

#[cfg(test)] 
mod test_common_coordinates {
    use super::*;

    #[test]
    fn it_returns_intersection() {
        let sets = vec![
            vec![1, 2, 3],
            vec![2, 3, 4],
        ].into_iter()
        .map(|vec| vec.into_iter().collect::<HashSet<_>>() )
        .collect::<Vec<HashSet<_>>>();

        let expected = vec![2, 3].into_iter().collect::<HashSet<_>>();

        let actual = common_coordinates(&mut sets.into_iter());
        assert_eq!(actual, Some(expected));
    }

    #[test]
    fn it_returns_empty_list() {
        let sets = vec![
            vec![1, 2, 3],
            vec![7, 8, 9],
        ].into_iter()
        .map(|vec| vec.into_iter().collect::<HashSet<_>>() )
        .collect::<Vec<HashSet<_>>>();

        // Intersection should be an empty set      
        let expected = vec![].into_iter().collect::<HashSet<_>>();

        let actual = common_coordinates(&mut sets.into_iter());
        assert_eq!(actual, Some(expected));
    }    
}


#[cfg(test)] 
mod test_only_place_it_can_go {
    use super::*;

    #[test]
    fn test_enough_free_ships_on_constant_axis() {
        let board = Board::new(&vec![
            "  0000", // deliberate: Don't have enough ships on incrementing axis
            "3|    ",
            "0|    ",
            "2| *  ",
        ]);

        // Enough space - No existing ships
        let coord = board.layout.coord(0, 0);
        let result = enough_free_ships_on_constant_axis(&board, 3, coord, Axis::Row, 0);
        assert_eq!(result, true);

        // Enough space - Includes an existing ships
        let coord = board.layout.coord(0, 2);
        let result = enough_free_ships_on_constant_axis(&board, 3, coord, Axis::Row, 1);
        assert_eq!(result, true);    

        // Not enough space
        let coord = board.layout.coord(0, 0);
        let result = enough_free_ships_on_constant_axis(&board, 4, coord, Axis::Row, 0);
        assert_eq!(result, false);   
    }

    #[test]
    fn test_enough_free_ships_on_incrementing_axis() {
        let board = Board::new(&vec![
            "  1110", 
            "0|    ", // deliberate: Don't have enough ships on constant axis
        ]);

        // Enough space - No existing ships
        let coord = board.layout.coord(0, 0);
        let result = enough_free_ships_on_incrementing_axis(&board, 3, coord, Axis::Col);
        assert_eq!(result, true);

        // Not enough space
        let coord = board.layout.coord(0, 0);
        let result = enough_free_ships_on_incrementing_axis(&board, 4, coord, Axis::Col);
        assert_eq!(result, false);

        let board = Board::new(&vec![
            "  1010", 
            "0| *  ", // deliberate: Don't have enough ships on constant axis
        ]);   

        // Enough space - Includes an existing ship
        let coord = board.layout.coord(0, 0);
        let result = enough_free_ships_on_incrementing_axis(&board, 3, coord, Axis::Col);
        assert_eq!(result, true);
    }    

    #[test]
    fn test_would_ship_at_coord_be_clear_of_other_ships() {
        let board = Board::new(&vec![
            "  0000",
            "0|    ",
            "0|~ ~ ",
            "0|< ~*",
            "0|~~~v",
        ]);    

        // Ship here would not touch another ship
        let coord = board.layout.coord(0, 0);
        let result = would_ship_at_coord_be_clear_of_other_ships(&board, 3, coord, Axis::Col);
        assert_eq!(result, true);

        // Ship here would diagonally touch the '<' at (0, 2)
        let coord = board.layout.coord(1, 0);
        let result = would_ship_at_coord_be_clear_of_other_ships(&board, 2, coord, Axis::Row);
        assert_eq!(result, false);

        // Ship here would touch the '*' at (3, 2)
        let coord = board.layout.coord(3, 0);
        let result = would_ship_at_coord_be_clear_of_other_ships(&board, 2, coord, Axis::Row);
        assert_eq!(result, false);
    }


    #[test]
    fn test_can_fit_ship_at_coord() {
        let board = Board::new(&vec![
            "  0000",
            "0|    ",
            "0|~ ~ ",
            "0|< ~*",
            "0|~~~v",
        ]);     

        // Can place it: All squares empty (non-dot ship)
        let coord = board.layout.coord(0, 0);
        let result = can_fit_ship_at_coord(&board, 3, coord, Axis::Col);
        assert_eq!(result, Some(0));

        // Can place it: All squares empty (dot ship)
        let coord = board.layout.coord(2, 0);
        let result = can_fit_ship_at_coord(&board, 1, coord, Axis::Col);
        assert_eq!(result, Some(0));    

        // Cannot place it: Water in the way
        let coord = board.layout.coord(0, 1);
        let result = can_fit_ship_at_coord(&board, 3, coord, Axis::Col);
        assert_eq!(result, None);

        // Can place it: Existing ships have the correct type
        let coord = board.layout.coord(3, 1);
        let result = can_fit_ship_at_coord(&board, 3, coord, Axis::Row);
        assert_eq!(result, Some(2));

        // Cannot place it: Existing ship has the wrong type
        let coord = board.layout.coord(0, 2);
        let result = can_fit_ship_at_coord(&board, 1, coord, Axis::Row);
        assert_eq!(result, None);           
    }

    #[test]
    fn test_can_fit_ship_doesnt_count_completed_ships() {
        let board = Board::new(&vec![
            "  1111",
            "1|<-->",
        ]);

        // Cannot place it: The entire ship is present
        let coord = board.layout.coord(0, 0);
        let result = can_fit_ship_at_coord(&board, 4, coord, Axis::Col);
        assert_eq!(result, None);
    }

    #[test]
    fn test_place_ship_at_coord() {
        let mut board = Board::new(&vec![
            "  002",
            "1|   ",
            "1| ~ ",
            "0|• *",
            "0|  v",
        ]);        
        let layout = board.layout;

        let mut changed = false;
        let coord = layout.coord(2, 0);
        place_ship_at_coord(&mut board, 4,  coord, Axis::Row, &mut changed);

        assert_eq!(true, changed);

        let expected = vec![
            "  000",
            "0|  ^",
            "0| ~|",
            "0|• |",
            "0|  v",
        ].iter().map(|x| x.to_string()).collect::<Vec<_>>();

        assert_eq!(board.to_strings(), expected); 
    }

    fn do_test(before: Vec<&str>, after: Vec<&str>) {
        let mut board = Board::new(&before);
        let expected = after.iter().map(|x| x.to_string()).collect::<Vec<_>>();

        let mut _changed = false;
        find_only_place_for_ships(&mut board, &mut _changed);

        let board_strings = board.to_strings();

        assert_eq!(board_strings, expected);

        // assert_eq!(board_strings.len(), expected.len());

        // let text_lines = board_strings.iter().zip(expected.iter());
        // for (actual_line, expected_line) in text_lines {
        //  assert_eq!(actual_line, expected_line);
        // }
    }

    // TESTS:
    // - We know where the middle of the ship will be but not the ends. Place ShipSquare::Any in the ones that we
    //   know will have a ship.

    #[test]
    fn it_fills_in_4sq() {
        do_test(vec![
            "ships: 4sq x 1.",
            "  01111",
            "4|~    ",
        ],
        vec![
            "ships: 4sq x 0.",
            "  00000",
            "0|~<-->",
        ]);
    }   

    #[test]
    fn doesnt_fill_in_adjacent_to_ship() {
        // Don't fill in the 4sq ship when it will abut an existing ship.
        // These are bad results:
        //
        // "ships: 4sq x 0.",
        // "  000000",
        // "4|~<-->*",
        //
        // "ships: 4sq x 0.",
        // "  000000",
        // "4|~*<-->",

        do_test(vec![
            "ships: 4sq x 1.",
            "  001110",
            "4|~*   *",
        ],
        vec![
            "ships: 4sq x 1.",
            "  001110",
            "4|~*   *",
        ]);
    }   


    #[test]
    fn it_fills_in_4sq_x2() {
        do_test(vec![
            "ships: 4sq x 2.",
            "  02222",
            "4|~    ",
            "0|~~~~~",
            "4|~    ",
        ],
        vec![
            "ships: 4sq x 0.",
            "  00000",
            "0|~<-->",
            "0|~~~~~",
            "0|~<-->",
        ]);
    }   

    #[test]
    fn it_doesnt_fill_if_not_enough_space() {
        do_test(vec![
            "ships: 4sq x 1.",
            "  01111",
            "3|~    ", // Only 3 ships to place here. Not enough room for the 4sq ship.
        ],
        vec![
            "ships: 4sq x 1.",
            "  01111",
            "3|~    ",
        ]);
    }

    #[test]
    fn it_fills_only_where_there_is_space_on_incrementing_axis() {
        do_test(vec![
            "ships: 2sq x 1.",
            "  00110",
            "2|~    ",
        ],
        vec![
            "ships: 2sq x 0.", 
            "  00000",
            "0|~ <> ", // These middle squares were the only ones with space on incrementing axis
        ]);
    }   

    #[test]
    fn it_fills_only_where_there_is_space_on_constant_axis() {
        do_test(vec![
            "ships: 3sq x 1.",
            "  030",
            "2|   ",
            "2|   ",
            "2|   ",
        ],
        vec![
            "ships: 3sq x 0.",
            "  000",
            "1| ^ ",
            "1| | ",
            "1| v ",
        ]);
    }       

    #[test]
    fn it_fills_in_dot() {
        do_test(vec![
            "ships: 1sq x 1.",
            "  00010",
            "1|~~~ ~",
        ],
        vec![
            "ships: 1sq x 0.",
            "  00000",
            "0|~~~•~",
        ]);
    }   

    #[test]
    fn it_fills_in_2_dots() {
        do_test(vec![
            "ships: 1sq x 2.",
            "  01010",
            "1|~~~ ~",
            "1|~ ~~~",
        ],
        vec![
            "ships: 1sq x 0.",
            "  00000",
            "0|~~~•~",
            "0|~•~~~",

        ]);
    }

    #[test]
    fn it_completes_partial_ship() {
        do_test(vec![
            "ships: 5sq x 1.",
            "  020",
            "1|~ ~",
            "0|~*~",
            "1|~ ~",
            "0|~*~",
            "0|~*~",            
            "0|~ ~",
        ],
        vec![
            "ships: 5sq x 0.",
            "  000",
            "0|~^~",
            "0|~|~",
            "0|~|~",
            "0|~|~",
            "0|~v~",            
            "0|~ ~",        
        ]);
    }

    #[test]
    fn it_completes_generic_middle_horizontally() {
        do_test(vec![
            "ships: 3sq x 1.",
            "  01010",
            "2|  ☐  ",
        ],
        vec![
            "ships: 3sq x 0.",
            "  00000",
            "0| <-> ",
        ]);
    }   

    #[test]
    fn it_completes_generic_middle_vertically() {
        do_test(vec![
            "ships: 3sq x 1.",
            "  020",
            "0|   ",
            "1|   ",
            "0| ☐ ",
            "1|   ",
            "0|   ",
        ],
        vec![
            "ships: 3sq x 0.",
            "  000",
            "0|   ",
            "0| ^ ",
            "0| | ",
            "0| v ",
            "0|   ",
        ]);
    }       

    #[test]
    fn it_places_partial_ship_when_one_possibility() {
        do_test(vec![
            "ships: 5sq x 1.",
            "  1111111",
            "7|       ",
        ],
        vec![
            "ships: 5sq x 1.",
            "  1100011",
            "4|  ***  ",
        ]);     
    }

    #[test]
    fn it_doesnt_place_partial_ship_when_two_possibilities() {
        do_test(vec![
            "ships: 5sq x 1.",
            "  1111111",
            "7|       ",
            "7|       ",
        ],
        vec![
            "ships: 5sq x 1.",
            "  1111111",
            "7|       ",
            "7|       ",
        ]);     
    }   


    #[test]
    fn it_places_two_partial_ships_when_two_possibilities() {
        do_test(vec![
            "ships: 5sq x 2.",
            "  2222222",
            "7|       ",
            "7|       ",
        ],
        vec![
            "ships: 5sq x 2.",
            "  2200022",
            "4|  ***  ",
            "4|  ***  ",
        ]);     
    }     
}

