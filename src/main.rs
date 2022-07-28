#![warn(clippy::all)]

use std::time::Instant;

use game::Game;
use log::error;
use pixels::{Error as PixelError, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

pub(crate) const WIDTH: u32 = 320;
pub(crate) const HEIGHT: u32 = 200;

mod game;

const NAME: &str = include_str!("name.txt");

fn main() -> Result<(), PixelError> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);

        WindowBuilder::new()
            .with_title(NAME)
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    // Game
    let mut game = Game::new();

    event_loop.run(move |event, _window_target, control_flow| {
        if let Event::RedrawRequested(_window_id) = event {
            game.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|err| {
                    error!(
                        "pixels.render() failed: {}\n --> {}:{}:{}",
                        err,
                        file!(),
                        line!(),
                        column!()
                    )
                })
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.quit() {
            *control_flow = ControlFlow::Exit;
            return;
        }

        if input.update(&event) {
            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            if !game.is_paused() {
                game.simulate_logic(Instant::now(), &mut input);
            }

            window.request_redraw();
        }
    });
}
