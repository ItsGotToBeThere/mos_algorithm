use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Query {
    index: usize,
    left_index_block: usize,
    left: usize,
    right: usize,
}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;
        match self.left_index_block.partial_cmp(&other.left_index_block) {
            Some(Equal) => {}
            ord => return ord,
        }
        self.right.partial_cmp(&other.right)
    }
}

impl Ord for Query {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => std::cmp::Ordering::Equal,
        }
    }
}

/// A struct containing the data used by out implementation
/// of Mo's Algorithm. Contains the data, and a max heap of
/// all the queries.
pub struct MosAlg {
    data: Vec<usize>,
    size: usize,
    block_size: usize,
    /// The binary heap is the max heap struct in rust.
    queries: BinaryHeap<Reverse<Query>>,
}

// Here are our function implementations!
impl MosAlg {
    /// I know yall are not super familiar with rust, so
    /// here's a quick crash course:
    /// The data lives in the structs. The implementations
    /// are applications of functions that act upon
    /// structs. Kinda like methods in java.
    /// If it has pub before it it is allowed to be used
    /// outside of the other implementations for the
    /// struct, kinda like private in java.
    /// The function signatures of the implementations can
    /// take in a `self`, kinda like python. However,
    /// unless it is a reference (`&self`), the `self` is
    /// consumed. So often when applying actions to
    /// structs we use a reference. When we want the
    /// implementation to modify the struct, we need to
    /// pass in a mutable reference, like `&mut self`.
    /// This implementation is like a constructor,
    /// returning an instance of Self.
    pub fn new(data: Vec<usize>) -> Self {
        let size = data.len();
        let block_size = (size as f32).sqrt().ceil() as usize;
        Self {
            data,
            size,
            block_size,
            queries: BinaryHeap::new(),
        }
    }

    pub fn add_query(&mut self, left: usize, right: usize) {
        self.queries.push(Reverse(Query {
            index: self.queries.len(),
            left_index_block: left / self.block_size,
            left,
            right,
        }));
    }

    pub fn execute(&self) -> Vec<usize> {
        let mut output = vec![0; self.queries.len()];
        let mut tmp = self.queries.clone();

        let mut start = 0;
        let mut end = 0;
        let mut lastblock = usize::MAX;
        let mut sum = 0;
        while let Some(Reverse(q)) = tmp.pop() {
            if q.left_index_block != lastblock {
                start = q.left;
                end = q.left;
                sum = self.data[q.left];
                while start > q.left {
                    start -= 1;
                    sum += self.data[start];
                }
                while start < q.left {
                    sum += 1;
                    sum -= self.data[start];
                }
                while end < q.right {
                    end += 1;
                    sum += self.data[end];
                }
                while end > q.right {
                    end -= 1;
                    sum -= self.data[end];
                }
                output[q.index] = sum;
            }
        }
        output
    }
}
