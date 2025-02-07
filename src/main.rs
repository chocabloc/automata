#![allow(dead_code)]

mod automata;
mod grid;
mod draw;
mod palette;

use raylib::prelude::RaylibDraw;
use automata::rules;
use grid::Grid;

fn main() {
    // create the window
    let (mut rl, thread) = raylib::init()
        .size(640, 640)
        .title("Automata")
        .build();
    rl.set_target_fps(0);

    // create grid
    let mut g = Grid::new(160, 160);
    g.set(80, 80, 8);
    let rule = rules::langton("LLRR");
    let palette = palette::langton("LLRR");
    
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        g.draw_to(&mut d, &palette);
        d.draw_fps(20, 20);
        g.step(&rule);
    }
}
