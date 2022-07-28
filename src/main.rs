#![warn(clippy::all)]

use pixels::{SurfaceTexture, Pixels, wgpu::Error, Error as PixelError};
use winit::{event_loop::EventLoop, dpi::LogicalSize, window::WindowBuilder, event::Event};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 200;

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

    event_loop.run(move |event, _window_target, control_flow| {
        if let Event::RedrawRequested(_window_id) = event {
            
        }
    });

    Ok(())
}
