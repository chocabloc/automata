pub type State = u64;
pub type Neighborhood = [[State; 3]; 3];
pub trait Rule: Fn(Neighborhood) -> State {}
impl<T: Fn(Neighborhood) -> State> Rule for T {}

#[allow(dead_code)]
pub mod rules {
    use super::{Neighborhood, State, Rule};

    // conway's game of life
    pub fn game_of_life(n: Neighborhood) -> State {
        let nn = n[0][0] + n[0][1] + n[0][2] + n[1][0] + n[1][2]
                 + n[2][0] + n[2][1] + n[2][2];
        
        if n[1][1] == 1 {
            match nn {
                0 | 1 => 0,
                2 | 3 => 1,
                _ => 0
            }
        }
        else if nn == 3 { 1 }
        else { 0 }
    }

    /* elementary cellular automata: y axis is time and 0 means uninitialised
        1 -> 0 and 2 -> 1. one row must be completely initialised or it no work
    */

    pub fn elementary(rn: u8) -> impl Rule {
        move |n| {
            if n[1][1] != 0 { n[1][1] }
            else if n[0][1] == 0 { 0 }
            else {
                let i = ((n[0][0] - 1) << 2) | ((n[0][1] - 1) << 1) | (n[0][2] - 1);
                (((1 << i) & rn as u64) >> i) + 1
            }
        }
    }

    /* cyclic automata: if a neighbour has next state, proceed to it
       otherwise stay same. n-1 proceeds to 0.
    */
    pub fn cyclic(states: u64) -> impl Rule {
        move |n| {
            let next_state = (n[1][1] + 1) % states;
            for row in n.iter() {
                for &cell in row.iter() {
                    if cell == next_state {
                        return next_state;
                    }
                }
            }
            n[1][1]
        }
    }
}