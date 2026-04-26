use std::collections::{BinaryHeap, HashMap};

mod query;
use query::Query;
pub use query::QueryType;

pub struct MosSolver {
    unique: usize,
    freq_query_threshold: usize,
    frequencies: HashMap<u32, usize>,
    queries: BinaryHeap<Query>,
    block_size: usize,
    data: Vec<u32>,
}

impl MosSolver {
    /// Initiates a new instance of the MosSolver struct, used to solve
    /// the PC using mos algorithm
    /// Takes in an array of IDs, `a`,
    /// and a threshold for the number of unique ids, `k`.
    pub fn init(a: &[u32], k: usize) -> Self {
        let block_size = (a.len() as f64).sqrt().ceil() as usize;
        Self {
            unique: 0,
            freq_query_threshold: k,
            frequencies: HashMap::new(),
            queries: BinaryHeap::new(),
            block_size,
            data: a.to_vec(),
        }
    }

    /// Inserts a query into the binary heap struct of the solver.
    /// When the solver runs, the results will be in the order that
    /// they were added in.
    pub fn insert_query(&mut self, qtype: QueryType, left: usize, right: usize) {
        self.queries.push(Query {
            index: self.queries.len(),
            query_type: qtype,
            left,
            block_index: left / self.block_size,
            right,
        });
    }

    fn add(&mut self, n: u32) {
        if let Some(freq) = self.frequencies.get_mut(&n) {
            *freq += 1;
            if *freq >= self.freq_query_threshold {
                self.unique += 1;
            }
        }
    }
    fn sub(&mut self, n: u32) {
        // TODO
    }
}
