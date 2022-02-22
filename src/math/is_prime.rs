pub mod is_prime {
    fn is_prime(x: usize) -> bool {
        let mut i = 2;
        while i * i <= x {
            if x % i == 0 {
                return false;
            }
        }
        true
    }
}
