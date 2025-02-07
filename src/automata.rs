pub type State = usize;
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
                (((1 << i) & rn as usize) >> i) + 1
            }
        }
    }

    /* cyclic automata: if a neighbour has next state, proceed to it
       otherwise stay same. n-1 proceeds to 0.
    */
    pub fn cyclic_moore(states: usize) -> impl Rule {
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

    pub fn cyclic_neumann(states: usize) -> impl Rule {
        move |n| {
            let next_state = (n[1][1] + 1) % states;
            if n[0][1] == next_state || n[1][0] == next_state ||
               n[1][2] == next_state || n[2][1] == next_state {
                return next_state;
            }
            n[1][1]
        }
    }

    pub fn avg(n: Neighborhood) -> State {
        let mut sum = 0;
        for row in n.iter() {
            for cell in row.iter() {
                sum += cell;
            }
        }
        (sum as f32 / 9.0).round() as State
    }

    // U -> R -> D -> L
    pub fn langton(name: &str) -> impl Rule {
        const DIRS: [u8; 5] = [b'N', b'U', b'R', b'D', b'L'];

        let nc = name.len();
        let nd = name.as_bytes().to_vec();

        move |n| -> State {
            let me = n[1][1] as usize;
            
            // are we an ant? is one of our neighbours an ant?
            if me/nc > 0 { (me+1)%nc }
            else {
                let (u, r, d, l) = (n[0][1], n[1][2], n[2][1], n[1][0]);
                let (ud, rd, dd, ld) = (u/nc, r/nc, d/nc, l/nc);
                let (uc, rc, dc, lc) = (u%nc, r%nc, d%nc, l%nc);

                let fd = if (DIRS[ud] == b'R' && nd[uc] == b'R') ||
                            (DIRS[ud] == b'L' && nd[uc] == b'L') { 3*nc }
                else if (DIRS[dd] == b'R' && nd[dc] == b'L') ||
                        (DIRS[dd] == b'L' && nd[dc] == b'R') { 1*nc }
                else if (DIRS[ld] == b'U' && nd[lc] == b'R') ||
                        (DIRS[ld] == b'D' && nd[lc] == b'L') { 2*nc }
                else if (DIRS[rd] == b'U' && nd[rc] == b'L') ||
                        (DIRS[rd] == b'D' && nd[rc] == b'R') { 4*nc }
                else { 0 };
                
                fd + (me%nc)
            }
        }
    }
}