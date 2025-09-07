use crate::{stack::*, stack_entry::StackEntry};
const LIMBO_BUFFER_SIZE: usize = 32;
pub struct Limbo {
    data: Stack<u8, LIMBO_BUFFER_SIZE>,
}

impl Limbo {
    pub fn new() -> Self {
        Self { data: Stack::new() }
    }

    pub fn push(&mut self, value: u8) -> Result<(), StackError> {
        self.data.push(value)
    }

    /// Pop a value off the stack.
    pub fn pop(&mut self) -> Result<u8, StackError> {
        self.data.pop()
    }

    /// Clears the stack.
    pub fn clear(&mut self) {
        self.data.clear()
    }

    /// Fills the stack with the given value.
    pub fn fill(&mut self, value: u8) {
        self.data.fill(value)
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Returns the current number of elements in the stack.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the stack is empty.
    pub fn empty(&self) -> bool {
        self.data.empty()
    }

    /// Returns true if the stack is full.
    pub fn full(&self) -> bool {
        self.data.full()
    }

    pub fn iter(&self) -> StackIter<u8> {
        self.data.iter()
    }

    pub fn parse(&self) -> Result<StackEntry, ()> {
        todo!("implement this")
    }
}
