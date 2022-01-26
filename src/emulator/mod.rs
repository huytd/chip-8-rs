use std::{fs::File, io::{Result, Read}};

use minifb::Key;

use self::cpu::CPU;

mod cpu;
mod display;
mod input;

pub struct Emulator {
    cpu: CPU,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
        }
    }

    pub fn set_keys(&mut self, keys: Vec<Key>) {
        let mapped_keys: Vec<u8> = keys.iter().map(|key| {
            match *key {
                Key::Key1 => 0x1,
                Key::Key2 => 0x2,
                Key::Key3 => 0x3,
                Key::Key4 => 0xc,
                Key::Q => 0x4,
                Key::W => 0x5,
                Key::E => 0x6,
                Key::R => 0xd,
                Key::A => 0x7,
                Key::S => 0x8,
                Key::D => 0x9,
                Key::F => 0xe,
                Key::Z => 0xa,
                Key::X => 0x0,
                Key::C => 0xb,
                Key::V => 0xf,
                _ => 0x0
            }
        }).collect();
        self.cpu.keys = mapped_keys;
    }

    pub fn run(&mut self) {
        self.cpu.run_timers();
        self.cpu.execute();
    }

    pub fn execute(&mut self) {
        self.cpu.execute();
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn load_rom(&mut self, path: &str) -> Result<()> {
        let mut f = File::open(path)?;
        let mut program = Vec::new();
        f.read_to_end(&mut program)?;
        self.cpu.load_program(&program);
        Ok(())
    }

    pub fn get_display_buffer(&self) -> &[u32] {
        self.cpu.display.get_buffer()
    }
}
