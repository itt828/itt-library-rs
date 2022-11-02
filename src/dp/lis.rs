//! 最長増加部分文字列を求める。
//! 返り値の$i$番目は、配列のi番目を最後の要素とするLISの長さ
//!

use super::super::data_structures::segtree::Segtree;
use cargo_snippet::snippet;

#[snippet]
fn lis(a: Vec<usize>) -> Vec<usize> {
    let n = a.len();
    let mut ret = Segtree::new();
    let mut ret = vec![0; n];
    let mut aa = a.iter().zip(0..n).collect::<Vec<_>>();
    aa.sort_by(|a, b| a.0.cmp(&b.0));
    for &(x, i) in &aa {
        ret[i] = ret.max(0..i) + 1;
    }
    ret
}
