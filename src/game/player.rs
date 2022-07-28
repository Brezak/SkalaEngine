use crate::{FractionNum, fraction_num::SignedFractionNum};

pub struct Player {
    pos_x: FractionNum,
    pos_y: FractionNum,
    dir_x: SignedFractionNum,
    dir_y: SignedFractionNum,
    cam_plane_x: SignedFractionNum,
    cam_plane_y: SignedFractionNum,
}

impl Player {
    pub const FRACTION_BITS: u64 = FractionNum::FRACTION_BITS;

    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos_x: 4.into(),
            pos_y: 4.into(),
            dir_x: (-1).into(),
            dir_y: 0.into(),
            cam_plane_x: 0.into(),
            cam_plane_y: 0.into(),
        }
    }
}
