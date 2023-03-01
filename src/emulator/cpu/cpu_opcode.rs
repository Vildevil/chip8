
#[derive(Debug)]
pub enum Opcode {
    Clear,
    Ret,
    Jump(usize),
    Call(usize),
    Skeq(usize, u8),
    Skne(usize, u8),
    Skreq(usize, usize),
    Ld(usize, u8),
    Add(usize, u8),
    Mov(usize, usize),
    Or(usize, usize),
    And(usize, usize),
    Xor(usize, usize),
    Addr(usize, usize),
    Sub(usize, usize),
    Shr(usize, usize),
    Skrne(usize, usize),
    Ldi(usize),
    Jmpi(usize),
    Rnd(usize, u8),
    Drw(usize, usize, usize),
    Skpr(usize),
    Skup(usize),
    Movdt(usize),
    Kwait(usize),
    Lddt(usize),
    Ldst(usize),
    Addi(usize),
    Lsdpr(usize),
    Bcd(usize),
    Stor(usize),
    Read(usize),
    Ignore,
}



pub fn parse_opcode(opcode: u16) -> Opcode {

    
    match opcode & 0xF000 {
        0x0000 => {
            match opcode & 0x00FF {
                0x00E0 => return Opcode::Clear,
                0x00EE => return Opcode::Ret,
                _ => return Opcode::Ignore,
            }
        }
        0x1000 => {
            return Opcode::Jump((opcode & 0x0FFF) as usize);
        },
        0x2000 => {
            return Opcode::Call((opcode & 0x0FFF) as usize);
        },
        0x3000 => {
            let vx = ((opcode & 0x0F00) >> 8u8) as usize; 
            let nn = (opcode & 0x00ff) as u8;

            return Opcode::Skeq(vx, nn);
        },
        0x4000 => {
            let vx = ((opcode & 0x0F00) >> 8u8) as usize; 
            let nn = (opcode & 0x00ff) as u8;

            return Opcode::Skne(vx, nn);
        },
        0x5000 => {
            let vx = (opcode & 0x0F00 >> 8u8) as usize;
            let vy = (opcode & 0x00F0 >> 4u8) as usize;
            return Opcode::Skreq(vx, vy);
        },
        0x6000 => {
            let vx = ((opcode & 0x0F00) >> 8u8) as usize;
            let nn = (opcode & 0x0ff) as u8;

            return Opcode::Ld(vx, nn);
        },
        0x7000 => {
            let vx = (opcode & 0x0F00 >> 8u8) as usize;
            let nn = (opcode & 0x0ff) as u8;

            return Opcode::Add(vx, nn);
        },
        0x8000 => {
            let vx = (opcode & 0x0F00 >> 8u8) as usize;
            let vy = (opcode & 0x00F0 >> 4u8) as usize;

            match opcode & 0x000F {
                0x0 => {
                    return Opcode::Mov(vx, vy);
                }
                0x1 => {
                    return Opcode::Or(vx, vy);
                },
                0x2 => {
                    return Opcode::And(vx, vy);
                },
                0x3 => {
                    return Opcode::Xor(vx, vy);
                },
                0x4 => {
                    return Opcode::Addr(vx, vy);
                },
                0x5 => {
                    return Opcode::Sub(vx, vy);
                },
                0x6 => {
                    return Opcode::Shr(vx, vy);
                },
                _ => {},
            }
        },
        0x9000 => {
            let vx = ((opcode & 0x0F00) >> 8u8) as usize;
            let vy = ((opcode & 0x00F0 )>> 4u8) as usize;

            return Opcode::Skrne(vx, vy);            
        },
        0xA000 => {
            return Opcode::Ldi((opcode & 0x0FFF) as usize);
        },
        0xB000 => {
            return Opcode::Jmpi((opcode & 0x0FFF) as usize);
        },
        0xC000 => {
            let vx = ((opcode & 0x0F00) >> 8u8) as usize;
            let nn = (opcode & 0x00FF) as u8;
            return Opcode::Rnd(vx, nn);
        },
        0xD000 => {
            let vx = ((opcode & 0x0F00) >> 8u8) as usize;
            let vy = ((opcode & 0x00F0) >> 4u8) as usize;

            let n: usize = (opcode & 0x000F) as usize;

            return Opcode::Drw(vx, vy, n);
        },
        0xE000 => {
            let vx = ((opcode & 0x0F00) >> 8u8) as usize;

            match opcode & 0x00FF {
                0x009E => {
                    return Opcode::Skpr(vx);
                },
                0x00A1 => {
                    return Opcode::Skup(vx);
                }
                _ => {}
            }
        },
        0xF000 => {
            let vx = ((opcode & 0x0F00)>> 8u8) as usize;

            match opcode & 0x00FF {
                0x0007 => {
                    return Opcode::Movdt(vx);
                },
                0x000A => {
                    return Opcode::Kwait(vx);
                },
                0x0015 => {
                    return Opcode::Lddt(vx);
                },
                0x0018 => {
                    return Opcode::Ldst(vx);
                },
                0x001E => {
                    return Opcode::Addi(vx);
                },
                0x0029 => {
                    return Opcode::Lsdpr(vx);
                },
                0x0033 => {
                    return Opcode::Bcd(vx);
                },
                0x0055 => {
                    return Opcode::Stor(vx);
                },
                0x0065 => {
                    return Opcode::Read(vx);
                }
                _ => {},
            }
        }
        _=> return Opcode::Ignore,
    }

    Opcode::Clear
}