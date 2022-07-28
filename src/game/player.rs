const FRACTION_BITS: u64 = 12;

pub struct Player {
    pos_x: u64,
    pos_y: u64,
    dir_x: i64,
    dir_y: i64,
    cam_plane_x: i64,
    cam_plane_y: i64,
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos_x: 4 << FRACTION_BITS,
            pos_y: 4 << FRACTION_BITS,
            dir_x: -1 << FRACTION_BITS,
            dir_y: 0,
            cam_plane_x: 0,
            cam_plane_y: 0,
        }
    }
}
