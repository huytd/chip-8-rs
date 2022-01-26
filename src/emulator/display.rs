use crate::consts::{WIDTH, HEIGHT, WHITE_PIXEL, BLACK_PIXEL};

pub static SPRITES: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Display {
    buffer: Vec<u32>
}

impl Display {
    pub fn new() -> Self {
        Self {
            buffer: vec![BLACK_PIXEL; WIDTH * HEIGHT]
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        self.buffer[x + y * WIDTH] = if on {
            WHITE_PIXEL
        } else {
            BLACK_PIXEL
        };
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> bool {
        return self.buffer[x + y * WIDTH] == WHITE_PIXEL
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let rows = sprite.len();
        let mut collision = false;
        for j in 0..rows {
            let row = sprite[j];
            for i in 0..8 {
                let val = row >> (7 - i) & 0x01;
                if val == 1 {
                    let xi = (x + i) % WIDTH;
                    let yj = (y + j) % HEIGHT;
                    let oval = self.get_pixel(xi, yj);
                    if oval {
                        collision = true;
                    }
                    self.set_pixel(xi, yj, (val == 1) ^ oval);
                }
            }
        }
        return collision;
    }

    pub fn get_buffer(&self) -> &[u32] {
        self.buffer.as_ref()
    }

    pub fn cls(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.set_pixel(x, y, false);
            }
        }
    }
}
