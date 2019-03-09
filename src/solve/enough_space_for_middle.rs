use crate::board::*;
use crate::error::*;
use crate::neighbor::*;
use crate::square::*;



// Determine if an AnyMiddle should be vertical or horizontal by looking at the
// number of ship squares remaining. 
pub fn enough_space_for_middle(board: &mut Board) -> Result<()> {
    let layout = board.layout;

    for coord in layout.all_coordinates() {
        if board[coord] != Square::ShipSquare(ShipSquare::AnyMiddle) {
            continue;
        }

        // Skip squares that are adjacent to board edges, or adjacent to a known square.
        // Other rules will fill those squares in.
        let neighbors = [Neighbor::N, Neighbor::E, Neighbor::S, Neighbor::W];
        let neighbors_ok = neighbors.iter().all(|neighbor| {
            match coord.neighbor(*neighbor) {
                Some(neighbor_coord) => board[neighbor_coord] == Square::Unknown,
                None => false,
            }
        });
        if !neighbors_ok {
            continue;
        }

        let has_space_in_row = board.ship_squares_remaining(coord.row()) >= 2;
        let has_space_in_col = board.ship_squares_remaining(coord.col()) >= 2;
        let ship_type_opt = match (has_space_in_row, has_space_in_col) {
            // The row contains enough space for a ship start and ship end,
            // but the col does not. This must be a horizontal ship.
            (true,  false) => Some(ShipSquare::HorizontalMiddle),

            // Must be a vertical ship
            (false, true)  => Some(ShipSquare::VerticalMiddle),

            // Ship could be either vertical or horizontal. Do nothing.
            (true,  true)  => None,

            // Error: Neither row nor col has enough space for a ship here
            (false, false) => {
                bail!("Ship middle has no space to become vertical or horizontal. {:?}", coord)
            }
        };

        if let Some(ship_type) = ship_type_opt {
            board.set(coord, Square::ShipSquare(ship_type))?;
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

        enough_space_for_middle(&mut board)?;
        assert_eq!(board.to_strings(), expected);     

        Ok(())   
    }

    #[test]
    fn it_specifies_vertical_middle() -> Result<()> {
        do_test(vec![
            // Only vertical axis has enough room
            "  00200",
            "0|     ",
            "1|  ☐  ", 
            "0|     ",
        ],
        vec![
            "  00200",
            "0|     ",
            "1|  |  ",
            "0|     ",
        ])
    }

    #[test]
    fn it_specifies_horizontal_middle() -> Result<()> {
        do_test(vec![
            // Only horizontal axis has enough room
            "  00100",
            "0|     ",
            "2|  ☐  ",
            "0|     ",
        ],
        vec![
            "  00100",
            "0|     ",
            "2|  -  ",
            "0|     ",
        ])
    }    

    #[test]
    fn it_doesnt_specify_next_to_ship() -> Result<()> {
        do_test(vec![
            "  00100",
            "0|     ",
            "2| *☐  ",
            "0|     ",
        ],
        vec![
            "  00100",
            "0|     ",
            "2| *☐  ",
            "0|     ",
        ])
    }       

    #[test]
    fn it_doesnt_specify_next_to_wall() -> Result<()> {
        do_test(vec![
            "  00100",
            "2|  ☐  ",
            "0|     ",
        ],
        vec![
            "  00100",
            "2|  ☐  ",
            "0|     ",
        ])
    }       
}   