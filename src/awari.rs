use std::ops::Index;

#[cfg(test)]
use quickcheck::{Arbitrary,Gen};

use utils;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Board4([u8; 8]);


impl Index<usize> for Board4 {
    type Output = u8;
    fn index(&self, i: usize) -> &u8 {
        let &Board4(ref b) = self;
        &b[i]
    }
}


impl Board4 {
    pub fn encode(&self) -> u64 {
        let mut c = self[0] as u64;
        let mut g = utils::binom(1, c);
        for i in 1..7 {
            c += 1 + self[i] as u64;
            g += utils::binom(i as u64 + 1, c);
        }
        let n = (0..8).fold(0, |a, i| a + self[i]) as u64;
        n + 25 * g
    }

    pub fn decode(g : u64) -> Self {
        let b = &mut [0; 8];
        let n : u64 = g % 25;
        let mut g : u64 = g / 25;

        let mut s = 0;
        let (mut c1, d) = utils::binom_maxinv(7, g);
        //println!("k: {}, g: {}, c: {}", 7, g, c1);
        //println!("c6: {}", c1);
        g -= d;
        for k in (0..6).rev() {
            let (c0, d) = utils::binom_maxinv(k as u64 + 1, g);
            //println!("k: {}, g: {}, c: {}", k, g, c0);
            g -= d;
            let a = c1 - c0 - 1;
            //println!("a: {}", a);
            s += a;
            b[k+1] = a as u8;
            c1 = c0;
        }
        b[0] = c1 as u8;
        b[7] = (n - s) as u8;
        println!("{:?}", *b);
        return Board4(*b);
    }
}


//TODO: more uniform...
#[cfg(test)]
impl Arbitrary for Board4 {
    fn arbitrary<G: Gen>(g: &mut G) -> Board4 {
        let b = &mut [0; 8];
        let mut s = 24u8;
        for i in 0..8 {
            let x = g.gen_range(0u8, s);
            b[i] = x;
            s -= x;
        }
        Board4(*b)
    }
}


#[cfg(test)]
quickcheck! {
    fn coding_bijective(b: Board4) -> bool {
        println!("{:?}", b);
        //println!("encoding: {}", b.encode());
        //println!("{:?}", Board4::decode(b.encode()));
        b == Board4::decode(b.encode())
    }
}
