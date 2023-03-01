use sdl2::{self, pixels::Color, rect};

use super::math::{Point, point};
use std::ops::Mul;

const PIXEL_BY_HEIGHT: usize = 32;
const PIXEL_BY_WIDTH: usize = 64;
const PIXEL_DIM: usize = 8;
const HEIGHT: u32 = (PIXEL_BY_HEIGHT * PIXEL_DIM) as u32;
const WIDTH: u32 = (PIXEL_BY_WIDTH * PIXEL_DIM) as u32;

const CLEAR_COLOR: Color = Color::BLACK;
const PIXEL_COLOR: Color = Color::WHITE;

#[derive(Clone, Copy)]
enum PixelSate {
    On,
    Off
}

pub enum Keycode {
    A,
    B,
}

pub enum SimpleEvents {
    Quit,
    KeyPressed(Keycode),
    KeyReleased(Keycode),
}


/// SDL Management structure
struct SDLHandle {
    context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
}



pub struct Screen {
    sdl: SDLHandle,
    screen_state: [[PixelSate; PIXEL_BY_HEIGHT]; PIXEL_BY_WIDTH]
}


impl SDLHandle {
    fn new() -> SDLHandle {
        let sdl_context = sdl2::init().unwrap();

        let sdl_video_subsystem = sdl_context.video().unwrap();

        let sdl_window = sdl_video_subsystem.window("Chip8", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let  sdl_canvas = sdl_window.into_canvas().build().unwrap();
        let  sdl_event_pump = sdl_context.event_pump().unwrap();

        SDLHandle { context: sdl_context , video_subsystem: sdl_video_subsystem, canvas: sdl_canvas, event_pump: sdl_event_pump }
    }

    /// Draw a pixel (scaled with PIXEL_DIM)
    fn draw_pixel(&mut self, position: Point) {
        self.canvas.set_draw_color(PIXEL_COLOR);
        let Point {x, y} = position;

        let pixel = sdl2::rect::Rect::new((x * 8) as i32, (y * 8) as i32, PIXEL_DIM as u32, PIXEL_DIM as u32);
        self.canvas.fill_rect(pixel).unwrap();
        
    }

    /// Alias for sdl canvas clear function
    fn clear(&mut self) {
        self.canvas.set_draw_color(CLEAR_COLOR);
        self.canvas.clear();
    }
    /// Alias for SDL2 canvas present function
    fn present(&mut self) {
        self.canvas.present();
    }
}



impl Screen {
    pub fn new() -> Screen {
        Screen { sdl: SDLHandle::new(), screen_state: [[PixelSate::Off; PIXEL_BY_HEIGHT]; PIXEL_BY_WIDTH] }
    }

    pub fn update() {

    }

    pub fn update_screen(&mut self) {
        
        self.sdl.clear();
        
        for x in 0..PIXEL_BY_WIDTH {
            for y in 0..PIXEL_BY_HEIGHT {
                if let PixelSate::On = self.screen_state[x][y] {
                    self.sdl.draw_pixel(point(x * PIXEL_DIM, y * PIXEL_DIM))
                };
            }
        }
        
        self.sdl.present();
    }

    pub fn update_pixel(&mut self, position: Point) {
        let Point { x, y } = position;

        self.screen_state[x][y] = match self.screen_state[x][y] {
            PixelSate::On => PixelSate::Off,
            PixelSate::Off => PixelSate::On,
        };
    }

    pub fn clear(&mut self) {
        for line in self.screen_state.iter_mut() {
            for pixel in line.iter_mut() {
                *pixel = PixelSate::Off;
            }
        }
    }
}