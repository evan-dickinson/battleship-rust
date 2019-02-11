/////////////////////////////////////////////////////////////////////
//
// Convert "any" ships to specific ships

use crate::board::*;
use crate::error::*;
use crate::square::*;

pub fn refine_any_ship_to_specific_ship(board: &mut Board) -> Result<()> {
    let layout = board.layout;
    for coord in layout.all_coordinates() {
        if board[coord] != Square::ShipSquare(ShipSquare::Any) {
            continue;
        }

        // Find the type of ship square (if any) that's the best fit for this coord
        let best_ship_square = ShipSquare::all()
            .filter(|&ship_square| {
                // let all_neighbors = Neighbor::all_neighbors();
                // let water_neighbors = ship_square.water_neighbors();
                // // TODO: If water_neighbors is a method on ship_square, then ship_neighbors should be, too.
                // let ship_neighbors = all_neighbors.difference(&water_neighbors);

                // Check ship_neighbors. Ensure they're both ships and in-bounds.
                // Can't use layout.coords_for_neighbors here because that filters out 
                // neigbors that are out of bounds.
                //
                // Ship neighbors need to be in bounds because those squares need to be
                // populated with ships. We don't want to set (0, 0) to the right end 
                // of a ship -- there's nowhere for the left end to go.
                let ship_neighbors_ok = ship_square.ship_neighbors().into_iter()
                    .all(|neighbor| coord.neighbor(neighbor)
                        .map_or(false, // out of bounds
                            |neighbor_coord| board[neighbor_coord].is_ship() 
                            )
                        );

                // Check that water neighbors are either out of bounds or set to water
                let water_neighbors_ok = ship_square.water_neighbors().into_iter()
                    .filter_map(|neighbor| coord.neighbor(neighbor))
                    .all(|water_coord| board[water_coord] == Square::Water);

                ship_neighbors_ok && water_neighbors_ok
            })
            .max_by_key(|ship_square| { 
                // If multiple ship_types match, choose the most specific type.
                // That's the one that sets the most surrounding squares to water.
                // Example: If both Dot and TopEnd match, prefer Dot.
                ship_square.water_neighbors().len() 
            });

        if let Some(ship_square) = best_ship_square {
            board.set(coord, Square::ShipSquare(ship_square))?
        }
    }

    Ok(())    
}

#[cfg(test)]
mod test {
    use super::*;

    fn do_test(before: Vec<&str>, after: Vec<&str>) -> Result<()> {
        let mut board = Board::new(&before)?;
        let expected = after.iter().map(|x| x.to_string()).collect::<Vec<_>>();

        refine_any_ship_to_specific_ship(&mut board)?;
        assert_eq!(board.to_strings(), expected); 
        Ok(())
    }

    #[test]
    fn it_creates_dot_surrounded_by_water() -> Result<()> {
        do_test(vec![
            "  00000",
            "0| ~~~ ",
            "0| ~*~ ",
            "0| ~~~ ",
        ],
        vec![
            "  00000",
            "0| ~~~ ",
            "0| ~•~ ",
            "0| ~~~ ",
        ])
    }

    #[test]
    fn it_creates_dot_in_corner() -> Result<()> {
        do_test(vec![
            "  000",
            "0| ~~",
            "0| ~*",
        ],
        vec![
            "  000",
            "0| ~~",
            "0| ~•",
        ])
    }   

    #[test]
    fn it_doesnt_create_dot_without_water_north() -> Result<()> {
        do_test(vec![
            "  000",
            "0| ~ ",
            "0| ~*",
        ],
        vec![
            "  000",
            "0| ~ ", 
            "0| ~*", // no change to dot, because north neighbor is unknown
        ])
    }       

    #[test]
    fn it_doesnt_create_dot_without_water_west() -> Result<()> {
        do_test(vec![
            "  000",
            "0| ~~",
            "0|  *",
        ],
        vec![
            "  000",
            "0| ~~", 
            "0|  *", // no change to dot, because west neighbor is unknown
        ])       
    }   

    #[test]
    fn it_creates_left_end_away_from_border() -> Result<()> {
        do_test(vec![
            "  00000",
            "0|~~~~ ",
            "0|~*-> ",
            "0|~~~~ ",
        ],
        vec![
            "  00000",
            "0|~~~~ ",
            "0|~<-> ",
            "0|~~~~ ",
        ])
    }   

    #[test]
    fn it_creates_left_end_at_border() -> Result<()> {
        do_test(vec![
            "  000",
            "0|~~~",
            "0|*> ",
            "0|~~~",            
        ],
        vec![
            "  000",
            "0|~~~",
            "0|<> ",
            "0|~~~",
        ]) 
    }

    #[test]
    fn it_creates_left_end_in_corner() -> Result<()> {
        do_test(vec![
            "  000",
            "0|*> ",
            "0|~~~",            
        ],
        vec![
            "  000",
            "0|<> ",
            "0|~~~",
        ])
    }

    #[test]
    fn it_creates_horizontal_middle_between_ends() -> Result<()> {
        do_test(vec![
            "  000",
            "0|~~~",
            "0|<*>",
            "0|~~~",
        ],
        vec![
            "  000",        
            "0|~~~",
            "0|<->",
            "0|~~~",
        ])
    }   

    #[test]
    fn it_creates_horizontal_middle_between_ends_on_border() -> Result<()> {
        do_test(vec![
            "  000",
            "0|<*>",
            "0|~~~",
        ],
        vec![
            "  000",        
            "0|<->",
            "0|~~~",
        ])
    }       

    #[test]
    fn it_doesnt_create_horizontal_middle_on_border_without_ends() -> Result<()> {
        do_test(vec![
            // Don't convert this ship. We can't tell if it will be an end,
            // a middle, or a dot
            "  000",
            "0| * ", 
            "0|~~~",
        ],
        vec![
            "  000",
            "0| * ", 
            "0|~~~",
        ])
    }   
}