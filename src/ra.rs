use std::cmp::max;

use awari::Awari;


#[derive(Serialize,Deserialize,Copy,Clone)]
pub enum State {
    Stable(i8),
    Unstable(i8, u8),
}

impl State {
    pub fn value(&self) -> i8 {
        match *self {
            State::Stable(s) => s,
            State::Unstable(s, _) => s,
        }
    }

    pub fn update(&mut self, up: i8) {
        match *self {
            State::Stable(s) => {
                assert!(s >= up)
            },
            State::Unstable(ref mut s, ref mut n) => {
                *n -= 1;
                if *s < up {
                    *s = up;
                }
            }
        }
    }

    pub fn flip_stable(&mut self, sat_lvl: i8) -> bool {
        match *self {
            State::Stable(_) => false,
            State::Unstable(s, n) => {
                if s == sat_lvl || n == 0 {
                    *self = State::Stable(s);
                    true
                } else { false }
            },
        }
    }
}

pub trait Storage  {
    fn new() -> Self;
    fn pre_row_hook(&mut self, usize);
    fn get(&self, usize) -> State;
    fn set(&mut self, usize, State);
}


fn propagate<T: Storage>(table: &mut T, u: Awari, up: i8, sat_lvl: i8) {
    let mut stack = vec![(u, up)];
    while let Some((u, up)) = stack.pop() {
        //info!("propagate: table[{}]=({}, {}, {}), score={}, sat={}", id, st.0,
        //        st.1, st.2, score, sat);
        let i = u.encode();
        let mut st = table.get(i);
        st.update(up);
        if st.flip_stable(sat_lvl) {
            let x = st.value();
            for v in u.predecessors() {
                stack.push((v, -x));
            }
        }
        table.set(i, st);
    }
}


pub fn analyze<T: Storage>(max_iter: usize) -> T {
    let mut table = T::new();
    table.pre_row_hook(0);
    table.set(0, State::Stable(0));
    
    for n in 1..max_iter+1 {
        println!("seed num {}", n);
        table.pre_row_hook(n);

        // initialization
        for u in Awari::iter_config(n) {
            let (mut score, mut nsucc) = (-(n as i8), 0);
            for (v, k) in u.successors() {
                if k > 0 {
                    score = max(score, k as i8 - table.get(v.encode()).value());
                }
                nsucc += 1;
            }
            table.set(u.encode(), State::Unstable(score, nsucc));
        }

        // convergence
        for l in 0..n+1 {
            info!("iteration {}", l);
            let sat_lvl = (n - l) as i8;
            for u in Awari::iter_config(n) {
                let i = u.encode();
                let mut state = table.get(i);
                if state.flip_stable(sat_lvl) {
                    let x = state.value();
                    table.set(i, state);
                    for v in u.predecessors() {
                        propagate(&mut table, v, -x, sat_lvl);
                    }
                }
            }
        }
    }
    
    return table;
}
