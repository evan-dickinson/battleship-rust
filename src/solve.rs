use crate::board::*;

mod fill_unknown;
mod surround_with_water;
mod specify_ships;
mod place_at_ends;
mod only_place_it_can_go;
mod specify_middles;
mod surround_middles;

pub fn solve(board : &mut Board) {
    let solvers = [
        self::fill_unknown::fill_with_water,
        self::fill_unknown::fill_with_ships,
        self::surround_with_water::surround_ships_with_water,
        self::place_at_ends::place_ships_next_to_ends,
        self::specify_ships::refine_any_ship_to_specific_ship,
        self::only_place_it_can_go::find_only_place_for_ships,
        self::specify_middles::specify_middle,
        self::surround_middles::surround_middle_with_ships,
    ];

    board.print();
    loop {
        let mut changed_in_loop = false;

        for solve in solvers.iter() {
            let mut changed_by_solver = false;
            solve(board, &mut changed_by_solver);
            if changed_by_solver {
                board.print();
            }

            changed_in_loop = changed_in_loop || changed_by_solver;
        }

        // If none of the solvers made a change, it's time to stop
        if !changed_in_loop {
            break;
        }
    }
}

