use awari;
use std::collections::HashMap;

enum State = {
    Instable(i8, u8),
    Stable(i8),
}

fn initialize(table: &[HashMap<awari::Board4, State>; 25], k: usize) -> HashMap<awari::Board4, State> {
    for
}

pub fn analysis() -> [HashMap<awari::Board4, State>; 25] {
    let mut table: [HashMap<awari::Board4, State>; 25];
    for k in 0..25 {
        for
        let mut curr = initialize(&table, k);
        converge(&mut curr, &table, k);
        table[k] = curr;
    }
}
