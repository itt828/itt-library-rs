pub struct CumulativeSum<T> {
    a: Vec<T>,
    calced: bool,
    sum: Vec<T>,
}

impl<T> CumulativeSum<T>
where
    T: Clone + Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    pub fn new(a: Vec<T>, e: T) -> Self {
        Self {
            a: a.clone(),
            calced: false,
            sum: vec![e; a.len() + 1],
        }
    }
    fn calc(&mut self) {
        self.calced = true;
        for i in 0..self.a.len() {
            self.sum[i + 1] = self.sum[i] + self.a[i];
        }
    }
    pub fn get(&mut self, l: usize, r: usize) -> T {
        if !self.calced {
            self.calc();
        }
        self.sum[r] - self.sum[l]
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn cumulative_sum_test() {
        let a = vec![1, 10, 100, 1000, 10000];
        let mut cumsum = crate::CumulativeSum::new(a, 0);
        assert_eq!(cumsum.get(2, 3), 100);
        assert_eq!(cumsum.get(0, 3), 111);
        assert_eq!(cumsum.get(2, 5), 11100);
        assert_eq!(cumsum.get(3, 4), 1000);
        assert_eq!(cumsum.get(0, 5), 11111);
    }
}
