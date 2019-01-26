use crate::square::*;
use crate::board::*;
use crate::neighbor::*;

// Add ships before/after a middle
pub fn surround_middle_with_ships(board: &mut Board, changed: &mut bool) {
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
        for neighbor in neighbors.into_iter() {
            // panic if neighbor_coord is out of bounds, because it means there's no space on the board
            // to place the neighboring ship squares. 
            let neighbor_coord = coord.neighbor(*neighbor).unwrap();

            if !board[neighbor_coord].is_ship() {
                board.set(neighbor_coord, Square::ShipSquare(ShipSquare::Any), changed);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn do_test(before: Vec<&str>, after: Vec<&str>) {
        let mut board = Board::new(&before);
        let expected = after.iter().map(|x| x.to_string()).collect::<Vec<_>>();

        let mut _changed = false;
        surround_middle_with_ships(&mut board, &mut _changed);
        assert_eq!(board.to_strings(), expected);        
    }

    #[test]
    fn it_specifies_vertical_middle_surrounded_by_water() {
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
        ]);
    }
}    