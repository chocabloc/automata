use std::{vec::Vec, mem};
use crate::automata::*;

pub struct Grid {
    g: Vec<State>,
    g0: Vec<State>,
    pub w: usize, pub h: usize
}

impl Grid {
    pub fn at(&self, mut x: i32, mut y: i32) -> State {
        let (wi, hi) = (self.w as i32, self.h as i32);
        x = x.rem_euclid(wi); y = y.rem_euclid(hi);
        self.g[(wi*y + x) as usize]
    }

    pub fn set(&mut self, x: usize, y: usize, s: State) {
        self.g[self.w*y + x] = s;
    }

    pub fn new(w: usize, h: usize) -> Self {
        Self { g: vec![0; w*h], g0: vec![0; w*h], w, h }
    }

    pub fn step(&mut self, rule: impl Rule) {
        let mut n: Neighborhood;
        let (wi, hi) = (self.w as i32, self.h as i32);

        for y in 0..hi {
            for x in 0..wi {
                n = [[self.at(x-1, y-1), self.at(x, y-1), self.at(x+1, y-1)],
                     [self.at(x-1, y), self.at(x, y), self.at(x+1, y)],
                     [self.at(x-1, y+1), self.at(x, y+1), self.at(x+1, y+1)]];
                self.g0[(wi*y + x) as usize] = rule(n);
            }
        }

        mem::swap(&mut self.g, &mut self.g0);
    }
}