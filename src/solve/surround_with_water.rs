/////////////////////////////////////////////////////////////////////
//
// Solutions that surround ships

use crate::board::*;
use crate::error::*;
use crate::square::*;

pub fn surround_ships_with_water(board: &mut Board) -> Result<()> {
    let layout = board.layout;
    let coords = layout.all_coordinates()
        .filter_map(|coord| { 
            if let Square::ShipSquare(ship_type) = board[coord] {
                // Return an iterator of the neighbors of coord that should be
                // set to water.
                let iter = ship_type.water_neighbors()
                    .into_iter()
                    .filter_map(move |neighbor| coord.neighbor(neighbor));

                Some(iter)
            }
            else {
                None
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    for coord in coords {
        board.set(coord, Square::Water)?;
    }

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    fn do_test(before: Vec<&str>, after: Vec<&str>) -> Result<()> {
        let mut board = Board::new(&before);
        let expected = after.iter().map(|x| x.to_string()).collect::<Vec<_>>();

        surround_ships_with_water(&mut board)?;
        assert_eq!(board.to_strings(), expected);        

        Ok(())
    }


    #[test]
    fn it_fills_diagonals() -> Result<()> {
        do_test(vec![
            "  00000",
            "0|     ",
            "0|     ",
            "0|  *  ",
            "0|     ",
            "0|     ",
        ],
         vec![
            "  00000",
            "0|     ",
            "0| ~ ~ ",
            "0|  *  ",
            "0| ~ ~ ",
            "0|     ",        
        ])
    }

    #[test]
    fn it_surrounds_dots() -> Result<()> {
        do_test(vec![
            "  00000",
            "0|     ",
            "0|  •  ",
            "0|     ",
        ],
        vec![
            "  00000",
            "0| ~~~ ",
            "0| ~•~ ",
            "0| ~~~ ",
        ])
    }

    #[test]
    fn it_surrounds_middles() -> Result<()> {
        do_test(vec![
            "  00000",
            "0|     ",
            "0|  -  ",
            "0|     ",
        ],
        vec![
            "  00000",
            "0| ~~~ ",
            "0|  -  ",
            "0| ~~~ ",
        ])
    }

    #[test]
    fn it_surrounds_ends() -> Result<()> {
        do_test(vec![
            "  00000",
            "0|     ",
            "0|  ^  ",
            "0|     ",
        ],
        vec![
            "  00000",
            "0| ~~~ ",
            "0| ~^~ ",
            "0| ~ ~ ",
        ])
    }


}




