
pub mod cpu;
pub mod screen;


mod math;

use self::cpu::{CPU, DrawInstruction};
use self::screen::Screen;

use std::thread;
use std::time::Duration;


/// Around 60 fps
const REFRESH_TIME: Duration = Duration::from_millis(16); 

pub struct Emulator {
    cpu: CPU,
    screen: Screen,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator { cpu: CPU::new(), screen: Screen::new() }
    }

    pub fn load(mut self, path: &str) -> Self {
        self.cpu.load_rom(path);
        self
    }

    pub fn run(mut self) {

        'main_loop: loop {

            match self.cpu.next() {
                DrawInstruction::SinglePixel(position) => self.screen.update_pixel(position),
                DrawInstruction::Sprite(positions) => {
                    for position in positions {
                        self.screen.update_pixel(position);
                    }
                },
                DrawInstruction::Clear => {
                    self.screen.clear();
                },
                _ => {}
            }
            self.screen.update_screen();
            thread::sleep(REFRESH_TIME);
        }
    }
}