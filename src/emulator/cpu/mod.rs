use std::fs;
use std::io;
use std::io::BufReader;
use std::io::Read;

use bitvec::prelude::*;

mod cpu_opcode;

use cpu_opcode::*;
use rand::random;
use super::math::{Point, point};


 /// The memory goes from 0x000 to 0xFFF 
const MEMORY_SIZE: usize = 4096;
/// The first 512 bytes was reserved for the interpreter
const START_ADDRESS: usize = 0x200;
const MAX_JUMP: usize = 16;
/// Each instruction is on 16 bits (2 bytes)
const INSTR_SIZE: usize = 2; 
pub enum DrawInstruction {
    None,
    Clear,
    SinglePixel(Point),
    Sprite(Vec<Point>),
}

pub struct CPU {
    v: [u8; 16],
    i: usize,
    memory: [u8; MEMORY_SIZE], 
    jump: [usize; MAX_JUMP],
    jump_nb: usize,
    sound_counter: u8,
    sys_counter: u8,
    pc: usize,
}



impl CPU {
    pub fn new() -> CPU {
        CPU {
            v: [0; 16], 
            i: 0,
            memory: [0; MEMORY_SIZE],
            jump: [0; MAX_JUMP],
            jump_nb: 0,
            sys_counter: 0,
            sound_counter: 0,
            pc: START_ADDRESS,
        }
    }


    /// Decrease CPU timer
    fn count(&mut self) {
        if self.sound_counter > 0 {
            self.sound_counter -= 1;
        }

        
        if self.sound_counter > 0 {
            self.sound_counter -= 1;
        }
    }


    /// Load the rom in the chip8 memory
    pub fn load_rom(&mut self, path: &str) {
        let f = fs::File::open(path).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).expect("Impossible to read the iso");

        for byte in buffer.iter() {
            self.memory[self.pc] = *byte;
            self.pc += 1;
        }

        self.pc = START_ADDRESS;
    }

    /// Public interface for read_opcode (no memory management)
    pub fn next(&mut self) -> DrawInstruction {
        let opcode = ((self.memory[self.pc] as u16)  << 8u8) + (self.memory[self.pc + 1]) as u16;
        let opcode = parse_opcode(opcode);
        println!("{:?}", opcode);

        match opcode {
            Opcode::Jump(nnn) => self.pc = nnn,
            Opcode::Call(nnn) => {
                if self.jump_nb < MAX_JUMP - 1 {
                    self.jump[self.jump_nb] = self.pc;
                    self.pc = nnn;
                    self.jump_nb += 1;
                }
            },
            Opcode::Skeq(vx, nn) => {
                if self.v[vx] == nn {
                    self.pc += INSTR_SIZE;
                }
            },
            Opcode::Skne(vx, nn) => {
                if self.v[vx] != nn {
                    self.pc += 2;
                }
            },
            Opcode::Skreq(vx, vy) => {
                if self.v[vx] == self.v[vy] {
                    self.pc += INSTR_SIZE;
                }
            },
            Opcode::Ld(vx, nn) => {
                self.v[vx] = nn;
            },
            Opcode::Add(vx, nn) => {
                self.v[vx] = self.v[vx].saturating_add(nn);
            },
            Opcode::Mov(vx, vy) => {
                self.v[vx] = self.v[vy];
            },
            Opcode::Or(vx, vy) =>  {
                self.v[vx] = self.v[vx] | self.v[vy];
            },
            Opcode::And(vx, vy) => {
                self.v[vx] = self.v[vx] & self.v[vy];
            },
            Opcode::Xor(vx, vy) => {
                self.v[vx] = self.v[vx] ^ self.v[vy];
            },
            Opcode::Addr(vx, vy) => {
                let x = self.v[vx];
                let y = self.v[vy];

                let (_, overflow) = x.overflowing_add(y);

                if overflow {
                    self.v[0xf] = 1;
                    self.v[vx] = x.saturating_add(y);
                }
                else {
                    self.v[0xf] = 0;
                    self.v[vx] = x + y;
                }
            },
            Opcode::Sub(vx, vy) => {
                let x = self.v[vx];
                let y = self.v[vy];

                let (_, overflow) = x.overflowing_sub(y);

                if overflow {
                    self.v[0xf] = 1;
                    self.v[vx] = x.saturating_sub(y);
                }
                else {
                    self.v[0xf] = 0;
                    self.v[vx] = x + y;
                }
            },
            Opcode::Shr(vx, _) => {
                let most_significant_bit = (0b10000000 & self.v[vx]) >> 7u8;                self.v[vx] = self.v[vx] << 1u8;
                self.v[0xf] = most_significant_bit;
            },
            Opcode::Skrne(vx, vy) => {
                if self.v[vx] != self.v[vy] {
                    self.pc += 2;
                }
            },
            Opcode::Ldi(nnn) => {
                self.i = nnn;
            },
            Opcode::Jmpi(nnn) => {
                self.pc = nnn + self.v[0] as usize;
            },
            Opcode::Rnd(vx, nn) => {
                use rand::Rng;

                let mut rng = rand::thread_rng();
                let random_value: u8 = rng.gen_range(1..=255);

                self.v[vx] = nn & random_value;
            },
            Opcode::Drw(vx, vy, n) => {
                let x = self.v[vx] as usize;
                let y = self.v[vy] as usize;
                let mut points: Vec<Point> = Vec::new();
                for i in 0..n {
                    let pixels = self.memory[self.i + i].view_bits::<Msb0>().to_bitvec();

                    for (index, pixel) in pixels.iter().enumerate() {
                        let pixel = *pixel;

                        if pixel {
                            points.push(point(x + index, y + i));
                        }
                    }
                }
                self.pc += INSTR_SIZE;
                return DrawInstruction::Sprite(points);
            },

            Opcode::Skpr(_) => {},
            Opcode::Skup(_) => {},
            Opcode::Movdt(vx) => self.v[vx] = self.sys_counter,
            Opcode::Kwait(_) => {},
            Opcode::Lddt(vx) => self.sys_counter = self.v[vx],
            Opcode::Ldst(vx) => self.sound_counter = self.v[vx],
            Opcode::Addi(vx) => {
                let x = self.v[vx] as usize;
                let (_, overflow) = self.i.overflowing_add(x);
                if overflow {
                    self.i = self.i.saturating_add(x);
                    self.v[0xf] = 1;
                }
                else {
                    self.v[0xf] = 0;
                    self.i += x;
                }
            },
            Opcode::Lsdpr(_) => {},
            Opcode::Bcd(vx) => {
                let nb = self.memory[vx];
                self.memory[self.i + 2] = nb / 100;
                self.memory[self.i + 1] = (nb / 10) % 10;
                self.memory[self.i] = nb % 10;
            }
            Opcode::Stor(vx) => {
                for index in 0..=vx {
                    self.memory[self.i + index] = self.v[index];
                }
            }
            Opcode::Read(vx) => {
                for index in 0..=vx {
                    self.v[index] = self.memory[self.i + index];
                }
            }

            _ => {}
        }
        self.pc += INSTR_SIZE;
        self.count();
        DrawInstruction::None
    }


}