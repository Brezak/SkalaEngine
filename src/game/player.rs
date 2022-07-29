use crate::{FractionNum, fraction_num::SignedFractionNum};

#[derive(Debug)]
pub struct Player {
    pub pos_x: SignedFractionNum,
    pub pos_y: SignedFractionNum,
    pub dir_x: SignedFractionNum,
    pub dir_y: SignedFractionNum,
    pub cam_plane_x: SignedFractionNum,
    pub cam_plane_y: SignedFractionNum,
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
