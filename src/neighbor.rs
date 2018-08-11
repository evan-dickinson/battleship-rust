
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Neighbor {
    N, NE, E, SE, S, SW, W, NW
}

impl Neighbor {
    pub fn all_neighbors() -> [Neighbor; 8] {
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

        return all_neighbors;
    }
}
