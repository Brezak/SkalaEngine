use crate::{fraction_num::SignedFractional, HEIGHT, WIDTH};
use std::time::{Duration, Instant};

use winit_input_helper::WinitInputHelper;

use self::player::Player;

mod player;

pub(crate) const WORLD_MAP: [[u8; 8]; 8] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1],
];

#[derive(Debug)]
pub struct Game {
    _last_time: Instant,
    _delta_time: Duration,
    player: player::Player,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn draw(&self, pixel_buffer: &mut [u8]) {
        #[derive(PartialEq, Eq)]
        enum WallHitSide {
            NS,
            EW, // ...gross
        }

        for x in 0..WIDTH {
            let camera_x = (SignedFractional::from_num(x) * 2) / SignedFractional::from_num(WIDTH)
                - SignedFractional::ONE; // x coordinate in camera space
            let ray_dir_x = self.player.dir_x + self.player.cam_plane_x * camera_x;
            let ray_dir_y = self.player.dir_y + self.player.cam_plane_y * camera_x;

            // Map tile position
            let mut map_x: usize = self.player.pos_x.to_num();
            let mut map_y: usize = self.player.pos_y.to_num();

            // Length of ray from current position to next x or y side
            let mut side_dist_x: SignedFractional;
            let mut side_dist_y: SignedFractional;

            // Length of ray from one x or y-side to next x or y side
            let delta_dist_x = if ray_dir_x == SignedFractional::ZERO {
                SignedFractional::from_num(i64::MAX)
            } else {
                (SignedFractional::from_num(1) / ray_dir_x).abs()
            };
            let delta_dist_y = if ray_dir_y == SignedFractional::ZERO {
                SignedFractional::from_num(i64::MAX)
            } else {
                (SignedFractional::from_num(1) / ray_dir_y).abs()
            };

            // What direction to step in x or y direction (either +1 or -1)
            let step_x: SignedFractional;
            let step_y: SignedFractional;

            let mut hit = false; // Did it hit a wall?
            let mut wall_hit_side = WallHitSide::NS;

            // Calculate step and initial side_dist
            if ray_dir_x < SignedFractional::ZERO {
                step_x = (-1).into();
                side_dist_x = (self.player.pos_x.frac()) * delta_dist_x;
            } else {
                step_x = 1.into();
                side_dist_x = (SignedFractional::ONE - self.player.pos_x.frac()) * delta_dist_x;
            }

            if ray_dir_y < SignedFractional::ZERO {
                step_y = (-1).into();
                side_dist_y = (self.player.pos_y.frac()) * delta_dist_y;
            } else {
                step_y = 1.into();
                side_dist_y = (SignedFractional::ONE - self.player.pos_y.frac()) * delta_dist_y;
            }

            // DDA
            while !hit {
                // Jump to next map tile, either in x direction or in y direction
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x.to_num::<usize>();
                    wall_hit_side = WallHitSide::NS;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y.to_num::<usize>();
                    wall_hit_side = WallHitSide::EW;
                }

                // Check if ray has hit a wall
                if WORLD_MAP[map_x][map_y] != 0 {
                    hit = true;
                }
            }

            let perp_wall_dist: SignedFractional = if wall_hit_side == WallHitSide::NS {
                side_dist_x - delta_dist_x
            } else {
                side_dist_y - delta_dist_y
            };

            // Calculate height of line to draw on screen
            let line_height: i64 = HEIGHT as i64 / perp_wall_dist.to_num::<i64>();

            // Calculate lowest and highest pixel to fill in current stripe
            let draw_bottom: usize = match -line_height / 2 + (HEIGHT as i64 / 2) {
                num @ 0.. => (num as usize),
                _ => 0,
            };

            const HEIGHT_I64: i64 = HEIGHT as i64;

            let draw_top: usize = match line_height / 2 + (HEIGHT as i64 / 2) {
                HEIGHT_I64.. => (HEIGHT - 1) as usize,
                num => (num as usize),
            };

            let mut color = match WORLD_MAP[map_x as usize][map_y as usize] {
                0 => [0u8, 0, 0, 0],
                _ => [255, 0, 0, 255],
            };

            // Give x and y sides different brightness
            if wall_hit_side == WallHitSide::EW {
                color[3] /= 2;
            }

            // Draw the pixels of the sprite as vertical line
            Self::draw_column(pixel_buffer, x as usize, draw_top, draw_bottom, color)
        }
    }

    fn draw_column(
        buffer: &mut [u8],
        column_index: usize,
        column_top: usize,
        column_bottom: usize,
        color: [u8; 4],
    ) {
        for (row, pixel) in buffer
            .chunks_exact_mut(4)
            .skip(column_index)
            .step_by(WIDTH as usize)
            .enumerate()
        {
            if row >= column_bottom && row <= column_top {
                pixel.copy_from_slice(color.as_slice());
            }
        }
    }

    pub fn simulate_logic(&mut self, _now: Instant, _input: &mut WinitInputHelper) {}

    pub fn is_paused(&self) -> bool {
        false
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            _last_time: Instant::now(),
            _delta_time: Duration::default(),
            player: Player::default(),
        }
    }
}
