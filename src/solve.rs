use itertools::Itertools;

use crate::board::*;
use crate::error::*;

mod fill_unknown;
mod surround_with_water;
mod specify_ships;
mod place_at_ends;
mod only_place_it_can_go;
mod specify_middles;
mod surround_middles;
mod enough_space_for_middle;

pub fn solve(board: &mut Board) -> Result<bool> {
    let solvers = [
        self::fill_unknown::fill_with_water,
        self::fill_unknown::fill_with_ships,
        self::surround_with_water::surround_ships_with_water,
        self::place_at_ends::place_ships_next_to_ends,
        self::specify_ships::refine_any_ship_to_specific_ship,
        self::only_place_it_can_go::find_only_place_for_ships,
        self::specify_middles::specify_middle,
        self::surround_middles::surround_middle_with_ships,
        self::enough_space_for_middle::enough_space_for_middle,
    ];

    board.print();
    loop {
        let is_changed = solvers.iter()
            .map(|solve| {
                board.clear_dirty();
                solve(board)?;

                if board.dirty() {
                    board.print()
                }

                // Compiler needs us to give a type annotation for the return type
                let result: Result<bool> = Ok(board.dirty());
                result
            })
            .fold_results(false, |acc, curr| acc || curr)?;

        // If none of the solvers made a change, it's time to stop
        if !is_changed {
            break;
        }
    }

    Ok(board.is_solved())
}
