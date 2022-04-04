pub fn is_prime(x: usize) -> bool {
    let mut i = 2;
    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

#[test]
fn is_prime_test() {
    assert!(is_prime(998244353));
    assert!(!is_prime(57));
}
