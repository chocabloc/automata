#![allow(dead_code)]

mod automata;
mod grid;
mod draw;
mod palette;

use automata::rules;
use grid::Grid;
use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    
    // create the window
    let (mut rl, thread) = raylib::init()
        .size(640, 640)
        .title("Automata")
        .build();
    rl.set_target_fps(60);

    // create grid                                                             
    let mut g = Grid::new(160, 160);
    for i in 0..160 {
        g.set(i, 0, rng.random_range(1..3));
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        g.draw_to(&mut d, palette::LCD);
        g.step(rules::elementary(110));
    }
}
