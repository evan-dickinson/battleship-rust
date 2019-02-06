use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Neighbor {
    N, NE, E, SE, S, SW, W, NW
}

impl Neighbor {
    pub fn all_neighbors() -> HashSet<Neighbor> {
        let all_neighbors = [
            Neighbor::N,
            Neighbor::NE,
            Neighbor::E,
            Neighbor::SE,
            Neighbor::S,
            Neighbor::SW,
            Neighbor::W,
            Neighbor::NW,
        ];

        all_neighbors.iter().cloned().collect()
    }

    pub fn all_except(exclude: Neighbor) -> HashSet<Neighbor> {
        Neighbor::all_neighbors().iter()
            .filter(|&x| *x != exclude)
            .cloned()
            .collect()
    }
}
