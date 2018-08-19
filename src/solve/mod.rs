use board::*;

mod fill_unknown;
use self::fill_unknown::*;

mod surround_with_water;
use self::surround_with_water::*;

mod specify_ships;
use self::specify_ships::*;

mod place_at_ends;
use self::place_at_ends::*;

mod only_place_it_can_go;
use self::only_place_it_can_go::*;

mod specify_middles;
use self::specify_middles::*;

mod surround_middles;
use self::surround_middles::*;

// TODO: Checks to implement:
// * Count ships (1 x size 3, 2 x size 2, etc.)
// * Refine a generic middle to vert or horiz, when surrounded by water

pub fn solve(board : &mut Board) {
    let solvers = [
        fill_with_water,
        fill_with_ships,
        surround_ships_with_water,
        place_ships_next_to_ends,
        specify_ships,
        find_only_place_for_ships,
        specify_middle,
        surround_middle_with_ships,
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
        if changed_in_loop == false {
            break;
        }
    }
}

