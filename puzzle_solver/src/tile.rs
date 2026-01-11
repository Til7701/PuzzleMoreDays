use crate::array_util::{debug_print, rotate_90};
use log::debug;
use ndarray::Array2;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tile {
    pub base: Array2<bool>,
    pub all_rotations: Vec<Array2<bool>>,
}

impl Tile {
    pub fn new(base: Array2<bool>) -> Tile {
        let mut all_rotations_set: HashSet<Array2<bool>> = HashSet::new();

        all_rotations_set.insert(base.clone());

        let mut tmp = rotate_90(&base);
        all_rotations_set.insert(tmp.clone());
        tmp = rotate_90(&tmp);
        all_rotations_set.insert(tmp.clone());
        tmp = rotate_90(&tmp);
        all_rotations_set.insert(tmp.clone());

        tmp = base.clone().reversed_axes();
        all_rotations_set.insert(tmp.clone());

        tmp = rotate_90(&tmp);
        all_rotations_set.insert(tmp.clone());
        tmp = rotate_90(&tmp);
        all_rotations_set.insert(tmp.clone());
        tmp = rotate_90(&tmp);
        all_rotations_set.insert(tmp.clone());

        let all_rotations = all_rotations_set.into_iter().collect();
        Tile {
            base,
            all_rotations,
        }
    }

    pub fn debug_print(&self) {
        debug!("Tile Base: ");
        debug_print(&self.base);
        debug!("All Rotations: ");
        for rotation in &self.all_rotations {
            debug!("Rotation:");
            debug_print(rotation);
        }
    }
}
