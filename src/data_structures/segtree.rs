use crate::internal::monoid::Monoid;

pub struct Segtree<M>
where
    M: Monoid<T>,
{
    n: usize,
    size: usize,
    log: usize,
    d: Vec<M::T>,
}

impl<M: Monoid> Segtree<M> {
    pub fn new(n: usize) -> Self {
        let mut log = 0;
        let mut size = 1;
        while size < n {
            log += 1;
            size *= 2;
        }
        let d = vec![M::id(); 2 * size];
        Segtree { n, size, log, d }
    }
    pub fn set(&mut self, p: usize, x: M::S) {
        self.d[p + self.size] = x;
        self.update(p);
    }
    pub fn get(&self, p: usize) -> M::S {
        self.d[p + self.size]
    }
    pub fn prod(&self, l: usize, r: usize) -> M::S {
        let mut l = l + self.size;
        let mut r = r + self.size;
        let mut vl = M::id();
        let mut vr = M::id();
        while l < r {
            if l & 1 == 1 {
                vl = M::op(&vl, &self.d[l]);
                l += 1;
            }
            if r & 1 == 1 {
                vr = M::op(&self.d[r], &vr);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(&vl, &vr)
    }
    fn update(&mut self, p: usize) {
        let mut p = p + self.size;
        let x = self.d[p];
        while p > 1 {
            p >>= 1;
            self.d[p] = M::op(&self.d[p << 1], &self.d[(p << 1) | 1]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Segtree;
    use crate::internal::impls;
    #[test]
    fn test_segtree() {
        let mut rsq = 
    }
}
