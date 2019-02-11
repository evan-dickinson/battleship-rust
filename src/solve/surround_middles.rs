use crate::board::*;
use crate::error::*;
use crate::layout::*;
use crate::neighbor::*;
use crate::square::*;


// Add ships before/after a middle
pub fn surround_middle_with_ships(board: &mut Board) -> Result<()> {
    let layout = board.layout;
    // Find all the middles, and identify which neighbors to set
    let coords_and_neighbors = layout.all_coordinates()
        .filter_map(|coord| {
            match board[coord] {
                Square::ShipSquare(ShipSquare::VerticalMiddle) => Some((
                    coord,
                    [Neighbor::N, Neighbor::S] // Set these neighbors to ships
                )),
                Square::ShipSquare(ShipSquare::HorizontalMiddle) => Some((
                    coord,
                    [Neighbor::E, Neighbor::W]
                )),
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    for (coord, neighbors) in coords_and_neighbors {
        for neighbor in neighbors.iter() {
            // TODO: This can become a method on coord. We use it in multiple places.
            //
            // Convert from Option<Coord> to Result<Coord>, so we can return an error
            // if neighbor is out of bounds. That would mean that, for example, the
            // top end of a ship is on the last row of the board. No place to put the
            // rest of the ship.
            let neighbor_coord_result: Result<Coord> = coord.neighbor(*neighbor)
                .ok_or_else(
                    || format!("Square {:?} at {:?} wants a neighbor to the {:?}, but no place to put it.",
                        board[coord], coord, neighbor).into()
                    );
            let neighbor_coord = neighbor_coord_result?;

            if !board[neighbor_coord].is_ship() {
                board.set(neighbor_coord, Square::ShipSquare(ShipSquare::Any))?;
            }
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

        surround_middle_with_ships(&mut board)?;
        assert_eq!(board.to_strings(), expected);     

        Ok(())   
    }

    #[test]
    fn it_specifies_vertical_middle_surrounded_by_water() -> Result<()> {
        do_test(vec![
            "  00200",
            "1|     ",
            "0| ~|~ ",
            "1|     ",
        ],
        vec![
            "  00000",
            "0|  *  ",
            "0| ~|~ ",
            "0|  *  ", 
        ])
    }
}    