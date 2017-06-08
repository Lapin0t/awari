use awari::Board4;
use std::collections::HashMap;
use std::vec::Vec;

type State = (i8, u8);
type Row = HashMap<Board4, State>;

pub fn analysis() -> Vec<Row> {
    let mut table = [(); 65739375];
    for n in 0..25 {
        let mut row = Row::new();

        // initialization
        for mut brd in Board4::iter_config(n) {
            let mut st = (-24, 4);
            for i in 0..4 {
                if !brd.is_valid(i) {
                    st.1 -= 1;
                } else {
                    let k = brd.play(i) as usize;
                    if k > 0 {
                        st.1 -= 1;
                        st.0 = k as i8 - table[n - k].get(&brd).unwrap().0;
                    }
                }
            }
            row.insert(brd, st);
        }

        // convergence
        for l in 0..n+1 {
            for brd in Board4::iter_config(n) {
                let &mut st = row.get_mut(&brd);
                if st.1 == 0 || st.0 == n - l {
                    propagate(&mut row, brd);
                }
            }
        }

        table[n] = row;
    }
    return table;
}
