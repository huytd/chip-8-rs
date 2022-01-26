use minifb::Scale;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const SCALE: Scale = Scale::X8;
const fn FROM_RGB(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
pub const WHITE_PIXEL: u32 = FROM_RGB(153, 255, 216);
pub const BLACK_PIXEL: u32 = FROM_RGB(32, 69, 55);
