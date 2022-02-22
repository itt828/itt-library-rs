pub mod cumulative_sum;
pub mod graph;
pub mod math;
pub mod strings;
pub mod util;
pub use cumulative_sum::CumulativeSum;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
