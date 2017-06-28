use std::cmp::max;
use std::boxed::{Box,HEAP};

use awari::Awari;
use super::{NBOARDS,SEEDS};



pub type Table = Box<[(i8, u8, bool); NBOARDS]>;


fn propagate(table: &mut Table, u: Awari, score: i8, sat: i8) {
    let mut stack = vec![(u, score)];
    while let Some((u, score)) = stack.pop() {
        let id = u.encode();
        let st = table[id];
        info!("propagate: table[{}]=({}, {}, {}), score={}, sat={}", id, st.0,
                st.1, st.2, score, sat);
        //assert!(!st.2);
        if !st.2 {
            table[id].1 -= 1;
            let s = max(st.0, -score);
            table[id].0 = s;
            if table[id].0 == sat || table[id].1 == 0 {
                table[id].1 = 0;
                table[id].2 = true;
                info!("STABLE: {} => {}", id, s);
                for v in u.predecessors() {
                    stack.push((v, s));
                }
            }
        } else {
            assert!(st.0 >= -score);
        }
    }
}


pub fn analysis() -> Table {
    let mut table = HEAP <- [(-(SEEDS as i8), 0, false); NBOARDS];
    table[0] = (0, 0, true);
    for n in 0..SEEDS+1 {
        println!("seed num {}", n);

        // initialization
        for u in Awari::iter_config(n) {
            let id = u.encode();
            table[id].0 = -(n as i8);
            for (v, k) in u.successors() {
                if k > 0 {
                    let score = k as i8 - table[v.encode()].0;
                    table[id].0 = max(table[id].0, score);
                }
                table[id].1 += 1;  // needs convergence
            }
        }

        // convergence
        for l in 0..n+1 {
            info!("iteration {}", l);
            let sat = (n - l) as i8;
            for u in Awari::iter_config(n) {
                let id = u.encode();
                let st = table[id];
                //println!("board: {}, score: {}, tbd: {}", id, st.0, st.1);
                if !st.2 && (st.0 == sat || st.1 == 0) {
                    table[id].1 = 0;
                    table[id].2 = true;
                    info!("STABLE: {} => {}", id, table[id].0);
                    for v in u.predecessors() {
                        propagate(&mut table, v, st.0, sat);
                    }
                }
            }
        }
    }
    return table;
}
