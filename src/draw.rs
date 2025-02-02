use raylib::prelude::*;
use crate::{palette::Palette, Grid};

impl Grid {
    pub fn draw_to(&self, d: &mut RaylibDrawHandle<'_>, p: impl Palette) {
        let (w, h) = (d.get_render_width(), d.get_screen_height());
        let (gw, gh) = (self.w as i32, self.h as i32);
        let (cw, ch) = (w/gw, h/gh);

        for y in 0..gh {
            for x in 0..gw {
                d.draw_rectangle(x*cw, y*ch, cw, ch, p(self.at(x, y)));
            }
        }
    }
}
