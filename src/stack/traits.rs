use core::ops::RangeBounds;

use crate::stack::StackError;

/// Basic operations for a generic stack.
///
/// Provides standard stack functionality such as pushing, popping, peeking,
/// checking emptiness/fullness, clearing, and filling the stack.
pub trait StackOps<T> {
    /// Pushes an element onto the stack.
    fn push(&mut self, item: T) -> Result<(), StackError>;

    /// Removes and returns the top element of the stack.
    fn pop(&mut self) -> Result<T, StackError>;

    /// Returns a reference to the top element without removing it.
    fn peek(&self) -> Option<&T>;

    /// Returns a slice of stack elements within the specified range.
    fn peek_range<R>(&self, range: R) -> Result<&[Option<T>], StackError>
    where
        R: RangeBounds<usize>;

    /// Returns `true` if the stack is empty.
    fn empty(&self) -> bool;

    /// Returns `true` if the stack is full.
    fn full(&self) -> bool;

    /// Removes all elements from the stack.
    fn clear(&mut self);

    /// Fills the stack with copies of the given element.
    fn fill(&mut self, item: T);
}

/// Advanced stack operations that manipulate elements beyond the top.
pub trait AdvancedStackOps<T> {
    /// Duplicates the top element of the stack.
    fn duplicate(&mut self) -> Result<(), StackError>;

    /// Deletes the element at the specified index, shifting elements down.
    fn delete_at(&mut self, index: usize) -> Result<T, StackError>;

    /// Swaps the top two elements of the stack.
    fn swap(&mut self) -> Result<(), StackError>;

    /// Pushes a copy of the second element from the top onto the stack.
    fn over(&mut self) -> Result<(), StackError>;

    /// Inserts a copy of the top element just below the second element.
    fn tuck(&mut self) -> Result<(), StackError>;
}
