use raylib::color::Color;
use crate::automata::State;
use rand::{Rng, SeedableRng};

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

pub fn grayscale(n: usize) -> impl Palette {
    move |s: State| {
        if n <= 1 { return Color::BLACK; }

        let step: f32 = 255.0/((n-1) as f32);
        let val = ((s as f32)*step) as u8;
        Color::new(val, val, val, 255)
    }
}

pub fn random(n: usize, seed: u64) -> impl Palette {
    let mut clr: Vec<Color> = vec![Color::WHITE; n as usize];
    let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);

    for i in 1..n as usize {
        let (r, g, b) = (rng.random_range(0..=255), rng.random_range(0..=255),
                         rng.random_range(0..=255));
        clr[i] = Color::new(r, g, b, 255)
    }

    move |s: State| { clr[s] }
}

pub fn langton(s: &str) -> impl Palette {
    let l = s.len();
    let g = random(l, 100);
    move |st: State| { g(st % l) }
}

pub const OLED: PaletteFn = binary::<0x000000ff, 0xffffffff>;
pub const LCD: PaletteFn = binary::<0xa8c64eff, 0x3c412cff>;
pub const SEPIA: PaletteFn = binary::<0xf5deb3ff, 0x704214ff>;
pub const INK: PaletteFn = binary::<0xddddddff, 0x222222ff>;
