use std::fs::File;
use std::io::{BufWriter,Result,Write};

use awari::{Awari,Board4};

pub fn write_dot(out: &str, n: usize) -> Result<()> {
    let mut stream = BufWriter::new(File::create(out)?);
    write!(&mut stream, "digraph foo {{\n")?;
    write!(&mut stream, "rankdir=\"LR\";\n")?;

    for i in (0..n).rev() {
        //write!(&mut stream, "subgraph cluster_{} {{\n", i)?;
        //write!(&mut stream, "label=\"{} seeds\";\n", i)?;
        for u in Board4::iter_config(i) {
            write!(&mut stream, "{id} [style=filled,label={id},color=\"/spectral11/{}\"];\n", i+1, id=u.encode())?;
        }
        //write!(&mut stream, "}}\n")?;
    }
    for i in 0..n {
        for u in Board4::iter_config(i) {
            for (v, k) in u.successors() {
                if k > 0 {
                    write!(&mut stream, "{} -> {} [color=red,label={}];\n", u.encode(), v.encode(), k)?;
                } else {
                    write!(&mut stream, "{} -> {};\n", u.encode(), v.encode())?;
                }
            }
        }
    }
    write!(&mut stream, "}}")
}
