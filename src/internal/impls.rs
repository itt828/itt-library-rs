use super::monoid::Monoid;

struct MonoidImpl<T: Copy, Id: Fn() -> T, Op: Fn(T, T) -> T>(pub Id, pub Op);

impl<T: Copy, Id: Fn() -> T, Op: Fn(T, T) -> T> Monoid<T> for MonoidImpl<T, Id, Op> {
    fn id(&self) -> T {
        (self.0)()
    }
    fn op(&self, a: T, b: T) -> T {
        (self.1)(a, b)
    }
}
