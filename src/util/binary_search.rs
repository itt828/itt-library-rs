pub mod binary_search {
    use std::cmp::Ordering;
    pub fn lower_bound<T: PartialOrd>(a: &Vec<T>, t: T) -> usize {
        let mut l = -1;
        let mut r = a.len() as isize;
        while r - l > 1 {
            let m = ((l + r) / 2) as usize;
            if t.partial_cmp(&a[m]).unwrap() != Ordering::Greater {
                r = m as isize;
            } else {
                l = m as isize;
            }
        }
        r as usize
    }
    pub fn upper_bound<T: PartialOrd>(a: &Vec<T>, t: T) -> usize {
        let mut l = -1;
        let mut r = a.len() as isize;
        while r - l > 1 {
            let m = ((l + r) / 2) as usize;
            if t.partial_cmp(&a[m]).unwrap() == Ordering::Less {
                r = m as isize;
            } else {
                l = m as isize;
            }
        }
        r as usize
    }
}
