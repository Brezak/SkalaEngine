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
            pos_x: 4.into(),
            pos_y: 4.into(),
            dir_x: SignedFractional::from_num(0),
            dir_y: SignedFractional::from_num(0),
            cam_plane_x: 0.into(),
            cam_plane_y: SignedFractional::from_num(0.66f64),
        }
    }
}
