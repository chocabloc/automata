mod automata;
mod grid;
mod draw;

use grid::Grid;
use automata::*;
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
    let mut g = Grid::new(320, 320);
    for i in 0..320 {
        g.set(i, 0, rng.random_range(1..3));
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        g.draw(&mut d);

        g.step(rules::rule::<110>);
    }
}
