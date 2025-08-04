use core::ops::RangeBounds;
pub trait StackOps<T, E> {
    fn push(&mut self, item: T) -> Result<(), E>;
    fn pop(&mut self) -> Result<T, E>;
    fn peek(&self) -> Option<&T>;
    fn peek_range<R>(&self, range: R) -> Result<&[Option<T>], E>
    where
        R: RangeBounds<usize>;
    fn empty(&self) -> bool;
    fn full(&self) -> bool;
    fn clear(&mut self);
    fn fill(&mut self, item: T);
}

pub trait AdvancedStackOps<T, E> {
    fn duplicate(&mut self) -> Result<(), E>;
    fn delete_at(&mut self, index: u8) -> Result<T, E>;
    fn swap(&mut self) -> Result<(), E>;
    fn over(&mut self) -> Result<(), E>;
    fn tuck(&mut self) -> Result<(), E>;
}
