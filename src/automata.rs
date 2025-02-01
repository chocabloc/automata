// types
pub type State = u64;
pub type Neighborhood = [[State; 3]; 3];
pub type Rule = fn(Neighborhood) -> State;

#[allow(dead_code)]
pub mod rules {
    use super::{Neighborhood, State};

    // conway's game of life
    pub fn gol(n: Neighborhood) -> State {
        let nn = n[0][0] + n[0][1] + n[0][2] + n[1][0] + n[1][2]
                 + n[2][0] + n[2][1] + n[2][2];
        
        if n[1][1] == 1 {
            match nn {
                0 | 1 => 0,
                2 | 3 => 1,
                _ => 0
            }
        } else {
            if nn == 3 { 1 } else { 0 }
        }
    }

    /*
        elementary cellular automata: y axis is time and 0 means uninitialised
        1 -> 0 and 2 -> 1. one row must be completely initialised or it no work
    */
    pub fn rule<const N: u8>(n: Neighborhood) -> State {
        if n[1][1] != 0 { n[1][1] }
        else {
            if n[0][1] == 0 { 0 }
            else {
                let i = ((n[0][0] - 1) << 2) | ((n[0][1] - 1) << 1) | (n[0][2] - 1);
                (((1 << i) & N as u64) >> i) + 1
            }
        }
    }
}