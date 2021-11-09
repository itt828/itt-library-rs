use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Clone, Copy)]
pub struct Edge {
    to: usize,
    cost: u64,
}

pub fn dijkstra(e: &Vec<Vec<Edge>>, from: usize) -> Vec<u64> {
    let mut ret = vec![1u64 << 60; e.len()];
    let mut q = BinaryHeap::<Reverse<(u64, usize)>>::new();
    q.push(Reverse((0, from)));
    while q.len() > 0 {
        let Reverse(cur) = q.pop().unwrap().clone();
        for edge in &e[cur.1] {
            if ret[edge.to] > if cur.1 == from { 0 } else { ret[cur.1] } + edge.cost {
                ret[edge.to] = if cur.1 == from { 0 } else { ret[cur.1] } + edge.cost;
                q.push(Reverse((ret[edge.to], edge.to)));
            }
        }
    }
    ret
}
