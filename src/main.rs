use std::env;

use consts::{WIDTH, HEIGHT, SCALE};
use minifb::{Window, WindowOptions, Key};

mod consts;
mod emulator;

use emulator::Emulator;

#[derive(Debug)]
struct EmulatorError;

impl From<minifb::Error> for EmulatorError {
    fn from(err: minifb::Error) -> Self {
        panic!("{:?}", err);
    }
}

impl From<std::io::Error> for EmulatorError {
    fn from(err: std::io::Error) -> Self {
        panic!("{:?}", err);
    }
}

fn main() -> Result<(), EmulatorError> {
    let args = env::args().collect::<Vec<String>>();
    let rom_path = match args.get(1) {
        Some(path) => path.to_owned(),
        None => "roms/OPCODE".to_owned()
    };
    let mut emulator = Emulator::new();
    emulator.reset();
    emulator.load_rom(&rom_path)?;

    let mut window = Window::new(
        "CHIP-8 Emulator",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: SCALE,
            ..WindowOptions::default()
        }
    )?;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        emulator.set_keys(window.get_keys());
        emulator.run();
        window.update_with_buffer(emulator.get_display_buffer(), WIDTH, HEIGHT)?;
    }

    Ok(())
}
