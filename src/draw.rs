use raylib::prelude::*;
use crate::Grid;

impl Grid {
    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        let (w, h) = (d.get_render_width(), d.get_screen_height());
        let (gw, gh) = (self.w as i32, self.h as i32);
        let (cw, ch) = (w/gw, h/gh);

        for y in 0..gh {
            for x in 0..gw {
                let s = self.at(x, y);

                // TODO: implement palletes
                let clr = match s {
                    0 => Color::BLACK,
                    1 => Color::WHITE,
                    _ => Color::BLACK
                };

                d.draw_rectangle(x*cw, y*ch, cw, ch, clr);
            }
        }
    }
}
