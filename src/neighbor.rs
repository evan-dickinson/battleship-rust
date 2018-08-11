
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Neighbor {
    N, NE, E, SE, S, SW, W, NW
}

impl Neighbor {
    pub fn all_neighbors() -> Vec<Neighbor> {
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

        return all_neighbors;
    }

    pub fn all_except(exclude : Neighbor) -> Vec<Neighbor> {
        return Neighbor::all_neighbors().iter()
            .filter(|&x| *x != exclude)
            .map(|x| *x)
            .collect::<Vec<Neighbor>>();
    }
}
