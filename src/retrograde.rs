use std::cmp::max;
use std::boxed::{Box,HEAP};

use awari::Board4;


pub type Table = Box<[(i8, u8); 10518300]>


fn propagate(table: &mut Table, brd: &Board4) {
    let st = table[brd.encode()];
    if st.1 == 0 || st.0 == n - l {

    }
}


pub fn analysis() -> Table {
    let mut table = HEAP <- [(-24, 4); 10518300];
    table[0] = (0, 0);
    for n in 1..25 {
        println!("seed num: {}", n);
        // initialization
        for mut brd in Board4::iter_config(n) {
            let mut st = table[brd.encode()];
            for i in 0..4 {
                if !brd.is_valid(i) {
                    st.1 -= 1;
                } else {
                    let k = brd.play(i) as usize;
                    if k > 0 {
                        st.1 -= 1;
                        let score = k as i8 - table[brd.encode()].0;
                        st.0 = max(st.0, score);
                    }
                }
            }
        }

        // convergence
        for l in 0..n+1 {
            for brd in Board4::iter_config(n) {
                let st = table[brd.encode()];
                if st.1 == 0 || st.0 == n - l {
                    propagate(&mut table, brd);
                }
            }
        }
    }
    return table;
}
