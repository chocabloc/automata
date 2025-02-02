use raylib::color::Color;
use crate::automata::State;

pub trait Palette: Fn(State) -> Color {}
impl<T: Fn(State) -> Color> Palette for T {}
type PaletteFn = fn(State) -> Color;

pub fn binary<const A: u32, const B: u32>(s: State) -> Color {
    match s {
        0 => Color::get_color(A),
        1 => Color::get_color(B),
        _ => Color::get_color(A)
    }
}

pub fn grayscale(n: u64) -> impl Palette {
    move |s| {
        if n <= 1 { return Color::BLACK; }

        let step: f32 = 255.0/((n-1) as f32);
        let val = ((s as f32)*step) as u8;
        Color::new(val, val, val, 255)
    }
}

pub const OLED: PaletteFn = binary::<0x000000ff, 0xffffffff>;
pub const INK: PaletteFn = binary::<0xffffffff, 0x000000ff>;
pub const LCD: PaletteFn = binary::<0xa8c64eff, 0x3c412cff>;