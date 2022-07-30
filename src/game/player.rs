use crate::fraction_num::SignedFractional;

#[derive(Debug)]
pub struct Player {
    pub pos_x: SignedFractional,
    pub pos_y: SignedFractional,
    pub dir_x: SignedFractional,
    pub dir_y: SignedFractional,
    pub cam_plane_x: SignedFractional,
    pub cam_plane_y: SignedFractional,
}

impl Player {
    pub fn _new() -> Self {
        Self::default()
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos_x: 2.into(),
            pos_y: 2.into(),
            dir_x: SignedFractional::from_num(-1),
            dir_y: SignedFractional::from_num(0),
            cam_plane_x: SignedFractional::from_num(0),
            cam_plane_y: SignedFractional::from_num(0.66),
        }
    }
}
