use std::time::{Duration, Instant};
use crate::{WIDTH, fraction_num::SignedFractionNum, HEIGHT};

use env_logger::fmt::Color;
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
    last_time: Instant,
    delta_time: Duration,
    player: player::Player,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn draw(&self, pixel_buffer: &mut [u8]) {

        #[derive(PartialEq, Eq)]
        enum WallHitSide{
            NS,
            EW // ...gross
        }

        for x in 0..WIDTH {
            let camera_x: SignedFractionNum = ((2 * x / WIDTH -1) as i64).into(); // x coordinate in camera space
            let ray_dir_x = self.player.dir_x + self.player.cam_plane_x * camera_x;
            let ray_dir_y = self.player.dir_y + self.player.cam_plane_y * camera_x;

            // Map tile position
            let mut map_x = self.player.pos_x.into_i64();
            let mut map_y = self.player.pos_y.into_i64();

            // Length of ray from current position to next x or y side
            let mut side_dist_x: SignedFractionNum;
            let mut side_dist_y: SignedFractionNum;

            // Length of ray from one x or y-side to next x or y side
            let delta_dist_x = if ray_dir_x == 0.into() {
                SignedFractionNum::new(i64::MAX)
            } else {
                (SignedFractionNum::new(1) / ray_dir_x).abs()
            };
            let delta_dist_y = if ray_dir_y == 0.into() {
                SignedFractionNum::new(i64::MAX)
            } else {
                (SignedFractionNum::new(1) / ray_dir_y).abs()
            };
            
            let perp_wall_dist: SignedFractionNum;

            // What direction to step in x or y direction (either +1 or -1)
            let step_x: SignedFractionNum;
            let step_y: SignedFractionNum;

            let mut hit = false; // Did it hit a wall?
            let mut wall_hit_side = WallHitSide::NS;

            // Calculate step and initial side_dist
            if ray_dir_x < 0.into(){
                step_x = (-1).into();
                side_dist_x = (self.player.pos_x - map_x) * delta_dist_x;
            } else {
                step_x = 1.into();
                side_dist_x = (SignedFractionNum::new(map_x + 1) - self.player.pos_x) * delta_dist_x;
            }

            if ray_dir_y < 0.into(){
                step_y = (-1).into();
                side_dist_y = (self.player.pos_y - map_y) * delta_dist_y;
            } else {
                step_y = 1.into();
                side_dist_y = (SignedFractionNum::new(map_y + 1) - self.player.pos_y) * delta_dist_y;
            }

            // DDA
            while hit == false {
                // Jump to next map tile, either in x direction or in y direction
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x.into_i64();
                    wall_hit_side = WallHitSide::NS;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y.into_i64();
                    wall_hit_side = WallHitSide::EW;
                }

                // Check if ray has hit a wall
                if WORLD_MAP[map_x as usize][map_y as usize] != 0 {
                    hit = true;
                }
            }

            if wall_hit_side == WallHitSide::NS {
                perp_wall_dist = side_dist_x - delta_dist_x;
            } else {
                perp_wall_dist = side_dist_y - delta_dist_y;
            }

            // Calculate height of line to draw on screen
            let line_height: i64 = HEIGHT as i64 / perp_wall_dist.into_i64();

            // Calculate lowest and highest pixel to fill in current stripe
            let mut draw_start: i64 = -line_height / 2 + (HEIGHT as i64 / 2);
            if draw_start < 0 {
                draw_start = 0;
            }

            let mut draw_end: i64 = line_height / 2 + (HEIGHT as i64 / 2);
            if draw_end >= HEIGHT.into() {
                draw_end = (HEIGHT - 1).into();
            }

            let mut color = match WORLD_MAP[map_x as usize][map_y as usize] {
                0 => [0u8, 0, 0, 0],
                _ => [255, 0 , 0, 255]
            };

            // Give x and y sides different brightness
            if wall_hit_side == WallHitSide::EW {
                color[0] /= 2;
            }

            // Draw the pixels of the sprite as vertical line
            
        }
    }

    pub fn simulate_logic(&mut self, now: Instant, input: &mut WinitInputHelper) {}

    pub fn is_paused(&self) -> bool {
        false
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            last_time: Instant::now(),
            delta_time: Duration::default(),
            player: Player::default()
        }
    }
}
