use rand::{thread_rng, Rng};

use super::display::{Display, SPRITES};

const MEMORY_SIZE: usize = 4096;
const REGISTERS_SIZE: usize = 16;
const STACK_SIZE: usize = 16;
const PROGRAM_OFFSET: usize = 0x200;

// Doc: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.0

pub struct CPU {
    // main memory
    pub memory: [u8; MEMORY_SIZE],
    // display
    pub display: Display,
    // keys
    pub keys: Vec<u8>,
    // stack
    stack: [u16; STACK_SIZE],
    // stack pointer
    sp: u8,
    // v registers
    v: [u8; REGISTERS_SIZE],
    // index registers
    i: u16,
    // delay timer
    dt: u8,
    // program counter
    pc: u16,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            display: Display::new(),
            keys: vec![],
            stack: [0; 16],
            sp: 0,
            v: [0; 16],
            i: 0,
            dt: 0,
            pc: 0,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for i in 0..program.len() {
            self.memory[PROGRAM_OFFSET + i] = program[i];
        }
    }

    pub fn reset(&mut self) {
        self.memory = [0; MEMORY_SIZE];
        self.display.cls();
        self.stack = [0; STACK_SIZE];
        self.sp = 0;
        self.v = [0; REGISTERS_SIZE];
        self.i = 0;
        self.dt = 0;
        self.pc = 0;
        for i in 0..80 {
            self.memory[i] = SPRITES[i];
        }
    }

    pub fn run_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
    }

    pub fn execute(&mut self) {
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[self.pc as usize + 1] as u16);

        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.v[x];
        let vy = self.v[y];
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        self.pc += 2;

        match (op_1, op_2, op_3, op_4) {
            // CLS
            (0, 0, 0xE, 0) => self.display.cls(),
            // RET
            (0, 0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            },
            // JP addr
            (0x1, _, _, _) => self.pc = nnn,
            // CALL addr
            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            // SE Vx, byte
            (0x3, _, _, _) => {
                if vx == kk {
                    self.pc += 2;
                }
            },
            // SNE Vx, byte
            (0x4, _, _, _) => {
                if vx != kk {
                    self.pc += 2;
                }
            },
            // SE Vx, Vy
            (0x5, _, _, 0) => {
                if vx == vy {
                    self.pc += 2;
                }
            },
            // LD Vx, byte
            (0x6, _, _, _) => self.v[x] = kk,
            // ADD Vx, byte
            (0x7, _, _, _) => self.v[x] = self.v[x].wrapping_add(kk),
            // LD Vx, Vy
            (0x8, _, _, 0) => self.v[x] = self.v[y],
            // OR Vx, Vy
            (0x8, _, _, 0x1) => self.v[x] = self.v[x] | self.v[y],
            // AND Vx, Vy
            (0x8, _, _, 0x2) => self.v[x] = self.v[x] & self.v[y],
            // XOR Vx, Vy
            (0x8, _, _, 0x3) => self.v[x] = self.v[x] ^ self.v[y],
            // ADD Vx, Vy
            (0x8, _, _, 0x4) => {
                let (ret, flag) = self.v[x].overflowing_add(self.v[y]);
                self.v[0xF] = if flag { 1 } else { 0 };
                self.v[x] = ret;
            },
            // SUB Vx, Vy
            (0x8, _, _, 0x5) => {
                let (ret, flag) = self.v[x].overflowing_sub(self.v[y]);
                self.v[0xF] = if flag { 0 } else { 1 };
                self.v[x] = ret;
            },
            // SHR Vx
            (0x8, _, _, 0x6) => {
                self.v[0xF] = self.v[x] & 0x1;
                self.v[x] >>= 1;
            },
            // SUBN Vx, Vy
            (0x8, _, _, 0x7) => {
                let (ret, flag) = self.v[y].overflowing_sub(self.v[x]);
                self.v[0xF] = if flag { 0 } else { 1 };
                self.v[x] = ret;
            },
            // SHL Vx
            (0x8, _, _, 0xE) => {
                self.v[0xF] = self.v[x] & 0x80;
                self.v[x] <<= 1;
            },
            // SNE Vx, Vy
            (0x9, _, _, _) => {
                if vx != vy {
                    self.pc += 2;
                }
            },
            // LD I, addr
            (0xA, _, _, _) => self.i = nnn,
            // JP V0, addr
            (0xB, _, _, _) => self.pc = nnn + self.v[0] as u16,
            // RND Vx, byte
            (0xC, _, _, _) => {
                let mut rng = thread_rng();
                let val: u8 = rng.gen();
                self.v[x] = val & kk;
            }
            // DRW
            (0xD, _, _, _) => {
                let sprite = &self.memory[self.i as usize .. (self.i + n as u16) as usize];
                let collision = self.display.draw(vx as usize, vy as usize, sprite);
                if collision {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
            },
            // SKP Vx
            (0xE, _, 0x9, 0xE) => {
                if self.keys.contains(&vx) {
                    self.pc += 2;
                }
            },
            // SKNP Vx
            (0xE, _, 0xA, 0x1) => {
                if !self.keys.contains(&vx) {
                    self.pc += 2;
                }
            },
            // LD Vx, DT
            (0xF, _, 0, 0x7) => self.v[x] = self.dt,
            // LD Vx, K
            (0xF, _, 0, 0xA) => {
                self.pc -= 2;
                if let Some(key) = self.keys.first() {
                    self.v[x] = *key;
                    self.pc += 2;
                }
            },
            // LD DT, Vx
            (0xF, _, 0x1, 0x5) => self.dt = vx,
            // LD ST, Vx - No sound support
            (0xF, _, 0x1, 0x8) => {},
            // ADD I, Vx
            (0xF, _, 0x1, 0xE) => self.i += self.v[x] as u16,
            // LD F, Vx
            (0xF, _, 0x2, 0x9) => self.i = vx as u16 * 5,
            // LD B, Vx
            (0xF, _, 0x3, 0x3) => {
                self.memory[self.i as usize] = vx / 100;
                self.memory[self.i as usize + 1] = (vx / 10) % 10;
                self.memory[self.i as usize + 2] = (vx % 100) % 10;
            },
            // LD [I], Vx
            (0xF, _, 0x5, 0x5) => self.memory[(self.i as usize)..(self.i + x as u16 + 1) as usize]
                        .copy_from_slice(&self.v[0..(x as usize + 1)]),
            // LD Vx, [I]
            (0xF, _, 0x6, 0x5) =>  self.v[0..(x as usize + 1)]
                        .copy_from_slice(&self.memory[(self.i as usize)..(self.i + x as u16 + 1) as usize]),
            _ => {}
        }
    }
}
