use crate::board::*;
use crate::error::*;
use crate::neighbor::*;
use crate::square::*;

// Determine if an AnyMiddle should be vertical or horizontal by looking at the
// number of ship squares remaining. 
pub fn enough_space_for_middle(board: &mut Board) -> Result<()> {
    let layout = board.layout;

    let to_set = layout.all_coordinates()
        .filter(|coord| board[*coord] == Square::ShipSquare(ShipSquare::AnyMiddle))
        .filter(|coord| {
            // Skip squares that are adjacent to board edges, or adjacent to a known square.
            // Other rules will fill those squares in.
            let neighbors = [Neighbor::N, Neighbor::E, Neighbor::S, Neighbor:: W];
            neighbors.iter().all(|neighbor| match coord.neighbor(*neighbor) {
                Some(neighbor_coord) => board[neighbor_coord] == Square::Unknown,
                None => false,
            })
        })
        .filter_map(|coord| {
            if board.ship_squares_remaining(coord.row()) < 2 {
                // Not enough space on the row for a horizontal ship, so this ship 
                // must be aligned vertically.
                Some((coord, ShipSquare::VerticalMiddle))
            }
            else if board.ship_squares_remaining(coord.col()) < 2 {
                // Likewise, but the ship must be horizontal
                Some((coord, ShipSquare::HorizontalMiddle))
            }
            else {
                None
            }
        })
        .collect::<Vec<_>>();

    for (coord, ship_type) in to_set {
        board.set(coord, Square::ShipSquare(ship_type))?
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