pub trait Monoid<T: Copy> {
    fn id() -> T;
    fn op(&self, a: T, b: T) -> T;
}
