mod mos;
use std::io::stdin;

use crate::mos::MosAlg;

fn main() {
    let mut lines = stdin().lines().filter_map(|x| x.ok());

    let array: Vec<usize> = lines
        .next()
        .expect("has array string")
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    let num_queries = lines
        .next()
        .expect("num queries line in input")
        .parse::<usize>()
        .expect("num queries input is a usize");

    let mut mos_solver = MosAlg::new(array);

    (0..num_queries).for_each(|_| {
        let parts: Vec<usize> = lines
            .next()
            .expect("query in input")
            .split_whitespace()
            .map(|index| index.parse().expect("range index is usize"))
            .take(2)
            .collect();
        mos_solver.add_query(parts[0], parts[1]);
    });

    for result in mos_solver.execute().into_iter() {
        println!("{result}");
    }
}
