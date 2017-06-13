use std::cmp::max;
use std::boxed::{Box,HEAP};

use awari::Board4;


pub type Table = Box<[(i8, u8); 10518300]>;


fn propagate(table: &mut Table, u: &Board4, score: i8, sat: i8) {
    let id = u.encode();
    let st = table[id];
    if st.1 > 0 {
        table[id].1 -= 1;
        let s = max(st.0, -score);
        table[id].0 = s;
        if table[id].0 == sat || table[id].0 == 0 {
            for v in u.predecessors() {
                propagate(table, &v, s, sat);
            }
        }
    } else {
        println!("ERROR: id={}; v-board={}, v-succ={}", id, st.0, score);
        assert!(st.0 >= -score);
    }
   /*
    let s_u = table[u.encode()];
    if s_u.0 == sat || s_u.1 == 0 {
        table[u.encode()].1 = 0;
        for v in u.predecessors() {
            let id = v.encode();
            if table[id].1 != 0 {
                //println!("foo");
                table[id].0 = max(table[id].0, -s_u.0);
                table[id].1 -= 1;
                propagate(table, &v, sat);
            } else {
                //println!("id: {}, {}, {}",id,  table[id].0, -s_u.0);
                // check we got something compatible
                if table[id].0 < -s_u.0 {
                    println!("incoherance: brd val={}, succ val={}", table[id].0, s_u.0);
                }
                //assert!(table[id].0 >= -s_u.0);
            }
        }
    }*/
}


pub fn analysis() -> Table {
    let mut table = HEAP <- [(-24, 0); 10518300];
    //table[0] = (0, 0);
    for n in 0..9 {
        println!("debug: seed num {}", n);

        // initialization
        for u in Board4::iter_config(n) {
            let id = u.encode();
            table[id].0 = -(n as i8);
            for (v, k) in u.successors() {
                if k > 0 {
                    let score = k as i8 - table[v.encode()].0;
                    table[id].0 = max(table[id].0, score);
                } else {
                    table[id].1 += 1;  // needs convergence
                }
            }
        }

        // convergence
        for l in 0..n+1 {
            for u in Board4::iter_config(n) {
                let id = u.encode();
                let sat = (n - l) as i8;
                let st = table[id];
                if st.0 == sat || st.1 == 0 {
                    table[id].1 = 0;
                    for v in u.predecessors() {
                        propagate(&mut table, &v, st.0, sat);
                    }
                }
            }
        }
    }
    return table;
}
