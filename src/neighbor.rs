use std::collections::HashSet;

use square::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Neighbor {
    N, NE, E, SE, S, SW, W, NW
}

impl Neighbor {
    pub fn all_neighbors() -> HashSet<Neighbor> {
        let all_neighbors = vec![
            Neighbor::N,
            Neighbor::NE,
            Neighbor::E,
            Neighbor::SE,
            Neighbor::S,
            Neighbor::SW,
            Neighbor::W,
            Neighbor::NW,
        ];

        return all_neighbors.iter().cloned().collect::<HashSet<Neighbor>>();
    }

    fn all_except(exclude : Neighbor) -> HashSet<Neighbor> {
        return Neighbor::all_neighbors().iter()
            .filter(|&x| *x != exclude)
            .map(|x| *x)
            .collect::<HashSet<Neighbor>>();
    }

    pub fn surrounding_neighbors(ship_type: Ship) -> HashSet<Neighbor> {
        return match ship_type {
            Ship::Any       => vec![
                Neighbor::NW, Neighbor::NE,
                Neighbor::SW, Neighbor::SE,
            ].iter().cloned().collect(),

            Ship::Dot       => Neighbor::all_neighbors(),

            Ship::LeftEnd   => Neighbor::all_except(Neighbor::E),
            Ship::RightEnd  => Neighbor::all_except(Neighbor::W),
            Ship::TopEnd    => Neighbor::all_except(Neighbor::S),
            Ship::BottomEnd => Neighbor::all_except(Neighbor::N),

            Ship::VerticalMiddle => vec![
                Neighbor::NE, Neighbor::E, Neighbor::SE,
                Neighbor::NW, Neighbor::W, Neighbor::SW,
            ].iter().cloned().collect(),
            Ship::HorizontalMiddle => vec![
                Neighbor::NW, Neighbor::N, Neighbor::NE,
                Neighbor::SW, Neighbor::S, Neighbor::SE,
            ].iter().cloned().collect(),
            Ship::AnyMiddle => vec![
                Neighbor::NW, Neighbor::NE,
                Neighbor::SW, Neighbor::SE,
            ].iter().cloned().collect(),
        };
    }

}
