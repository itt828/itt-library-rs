pub mod wavelet_matrix {
    use crate::util::binary_search::binary_search::lower_bound;
    #[derive(Debug)]
    pub struct BitVector {
        x: Vec<usize>,
        sum1: Vec<usize>,
        sum0: Vec<usize>,
    }
    impl BitVector {
        pub fn new(x: Vec<usize>) -> Self {
            let mut sum1 = vec![0; x.len()];
            let mut sum0 = vec![0; x.len()];
            sum1[0] = x[0];
            sum0[0] = 1 - x[0];
            for i in 1..x.len() {
                sum1[i] = sum1[i - 1] + x[i];
                sum0[i] = sum0[i - 1] + 1 - x[i];
            }
            Self { x, sum1, sum0 }
        }
        pub fn access(&self, i: usize) -> usize {
            self.x[i]
        }
        pub fn rank1(&self, i: usize, j: usize) -> usize {
            self.sum1[j] - if i == 0 { 0 } else { self.sum1[i - 1] }
        }
        pub fn rank0(&self, i: usize, j: usize) -> usize {
            self.sum0[j] - if i == 0 { 0 } else { self.sum0[i - 1] }
        }
        pub fn select0(&self, k: usize) -> usize {
            lower_bound(&self.sum0, k)
        }
        pub fn select1(&self, k: usize) -> usize {
            lower_bound(&self.sum1, k)
        }
    }
    #[derive(Debug)]
    pub struct WaveletMatrix {
        pub b: Vec<BitVector>,
        pub mid: Vec<usize>,
    }
    const MAX_BIT: usize = 3;
    impl WaveletMatrix {
        pub fn new(s: &Vec<usize>) -> Self {
            let mut b = vec![];
            let mut mid = vec![];
            let mut cur_s = s.clone();
            let mut l_vec = vec![];
            let mut r_vec = vec![];
            let mut cur_v = vec![];
            let mut tmp_cnt = 0;
            for &x in &cur_s {
                if (x >> MAX_BIT - 1) & 1 == 1 {
                    r_vec.push(x);
                    cur_v.push(1);
                } else {
                    l_vec.push(x);
                    cur_v.push(0);
                    tmp_cnt += 1;
                }
            }
            cur_s = vec![];
            for &x in &l_vec {
                cur_s.push(x);
            }
            for &x in &r_vec {
                cur_s.push(x);
            }
            b.push(BitVector::new(cur_v));
            mid.push(tmp_cnt);

            for i in (0..=MAX_BIT - 2).rev() {
                l_vec = vec![];
                r_vec = vec![];
                cur_v = vec![];
                tmp_cnt = 0;
                for &x in &cur_s {
                    if (x >> i) & 1 == 1 {
                        r_vec.push(x);
                        cur_v.push(1);
                    } else {
                        l_vec.push(x);
                        cur_v.push(0);
                        tmp_cnt += 1;
                    }
                }
                cur_s = vec![];
                for &x in &l_vec {
                    cur_s.push(x);
                }
                for &x in &r_vec {
                    cur_s.push(x);
                }
                b.push(BitVector::new(cur_v));
                mid.push(tmp_cnt);
            }
            Self { b, mid }
        }
        pub fn access(&self, mut i: usize) -> usize {
            let mut ret = 0;
            for j in 0..MAX_BIT {
                if self.b[j].access(i) == 1 {
                    i = self.mid[j] + self.b[j].rank1(0, i) - 1;
                    ret += 1 << (MAX_BIT - 1 - j);
                } else {
                    i = self.b[j].rank0(0, i) - 1;
                }
            }
            ret
        }
        pub fn rank(&self, c: usize, mut i: usize) -> usize {
            let mut p = 0;
            for j in 0..MAX_BIT {
                if (c >> (MAX_BIT - 1 - j)) & 1 == 1 {
                    p = self.mid[j] + self.b[j].rank1(0, p) - 1;
                    i = self.mid[j] + self.b[j].rank1(0, i) - 1;
                } else {
                    p = self.b[j].rank0(0, p) - 1;
                    i = self.b[j].rank0(0, i) - 1;
                }
            }
            i - p + 1
        }
        pub fn select(&self, c: usize, j: usize) -> usize {
            //cが最初に現れるインデックスを求める.
            let mut i = 0;
            for k in 0..MAX_BIT {
                if (c >> (MAX_BIT - 1 - k)) & 1 == 1 {
                    i = self.mid[k] + self.b[k].rank1(0, i) - 1;
                } else {
                    i = self.b[k].rank0(0, i) - 1;
                }
            }
            i += j - 1;
            //下から、求める値がある位置を保持しながらのぼってくる
            for k in (0..MAX_BIT).rev() {
                println!("{}", i);
                if i < self.mid[k] {
                    i = self.b[k].select0(i + 1);
                    println!("{},{}", k, i);
                } else {
                    i = self.b[k].select1(i - self.mid[k] + 1);
                }
            }
            i
        }
    }
}

#[cfg(test)]
mod tests {
    use super::wavelet_matrix;

    #[test]
    fn wavelet_matrix_test() {}
}
