use crate::stack::AdvancedStackOps;

use super::traits::StackOps;
use core::{fmt, ops::RangeBounds};

// #region Stack
#[derive(Debug)]
pub struct Stack<T, const N: usize> {
    data: [Option<T>; N],
    top: Option<usize>,
}

impl<T, const N: usize> Stack<T, N>
where
    T: Copy,
{
    /// Creates a new, empty stack.
    pub fn new() -> Self {
        // Safety ruling out the extemes of usize to prevent headache
        debug_assert!(N > 0, "Stack size N must be > 0");
        debug_assert!(N < usize::MAX, "Stack size N must be < usize::MAX");
        Stack {
            data: [None; N],
            top: None,
        }
    }

    /// Creates a stack pre-filled with the given entries.
    pub fn with<const AMOUNT: usize>(entries: [T; AMOUNT]) -> Result<Self, StackError> {
        // Safety ruling out the extemes of usize to prevent headache
        debug_assert!(N > 0, "Stack size N must be > 0");
        debug_assert!(N < usize::MAX, "Stack size N must be < usize::MAX");
        if AMOUNT > N {
            return Err(StackError::Overflow);
        }

        let mut data: [Option<T>; N] = [None; N];
        for (i, entry) in entries.into_iter().enumerate() {
            data[i] = Some(entry);
        }
        let top = if AMOUNT == 0 { None } else { Some(AMOUNT - 1) };

        Ok(Stack { data, top })
    }

    /// Returns the maximum capacity of the stack.
    pub fn capacity(&self) -> usize {
        N
    }

    /// Returns the current number of elements in the stack.
    pub fn len(&self) -> usize {
        match self.top {
            None => 0,
            Some(i) => i + 1,
        }
    }
}

impl<T, const N: usize> StackOps<T> for Stack<T, N>
where
    T: Copy,
{
    fn push(&mut self, entry: T) -> Result<(), StackError> {
        let next = match self.top {
            None => 0,
            Some(i) if i + 1 < N => i + 1,
            Some(_) => return Err(StackError::Overflow),
        };
        self.data[next] = Some(entry);
        self.top = Some(next);
        Ok(())
    }

    fn pop(&mut self) -> Result<T, StackError> {
        match self.top {
            None => Err(StackError::Underflow),
            Some(i) => {
                let val = self.data[i].take().ok_or(StackError::Underflow)?;

                self.top = if i == 0 { None } else { Some(i - 1) };
                Ok(val)
            }
        }
    }

    fn peek(&self) -> Option<&T> {
        match self.top {
            None => None,
            Some(i) => self.data[i].as_ref(),
        }
    }

    fn peek_range<R>(&self, range: R) -> Result<&[Option<T>], StackError>
    where
        R: RangeBounds<usize>,
    {
        use core::ops::Bound;

        let start = match range.start_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i.checked_add(1).ok_or(StackError::InvalidStartIndex(i))?,
            Bound::Unbounded => 0,
        };

        let top_index = match self.top {
            Some(i) => i,
            None => return Err(StackError::Empty),
        };

        let end = match range.end_bound() {
            Bound::Included(&i) => i.checked_add(1).ok_or(StackError::InvalidEndIndex(i))?,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => top_index
                .checked_add(1)
                .ok_or(StackError::InvalidEndIndex(top_index))?,
        };

        if start > end || end > top_index + 1 {
            return Err(StackError::InvalidRange(start, end, top_index));
        }

        Ok(&self.data[start..end])
    }

    fn empty(&self) -> bool {
        match self.top {
            None => true,
            Some(_) => false,
        }
    }

    fn full(&self) -> bool {
        self.len() == N
    }

    fn clear(&mut self) {
        self.data.fill(None);
        self.top = None;
    }

    fn fill(&mut self, item: T) {
        self.data.fill(Some(item));
        self.top = Some(N - 1);
    }
}

impl<T, const N: usize> AdvancedStackOps<T> for Stack<T, N>
where
    T: Copy,
{
    fn duplicate(&mut self) -> Result<(), StackError> {
        match self.top {
            None => Err(StackError::Empty),
            Some(i) => {
                if i + 1 >= N {
                    return Err(StackError::Overflow);
                }
                match self.data[i].as_ref() {
                    Some(entry) => self.push(*entry),
                    None => Err(StackError::Invalid("Corrupted stack: no Entry at top")),
                }
            }
        }
    }

    fn delete_at(&mut self, index: usize) -> Result<T, StackError> {
        let idx = index;

        if self.empty() {
            return Err(StackError::Empty);
        }
        let top_idx = self
            .top
            .expect("expected 'top' to be 'some' and not 'none'") as usize;
        if idx > top_idx {
            return Err(StackError::InvalidEndIndex(idx));
        }

        match self.data[idx].take() {
            Some(entry) => {
                for i in idx..top_idx {
                    self.data[i] = self.data[i + 1].take();
                }
                self.data[top_idx] = None;
                self.top = if top_idx == 0 {
                    None
                } else {
                    Some(top_idx - 1)
                };
                Ok(entry)
            }
            None => Err(StackError::Empty),
        }
    }

    fn swap(&mut self) -> Result<(), StackError> {
        let top = match self.top {
            None => return Err(StackError::Empty),
            Some(0) => return Err(StackError::NotEnoughElements),
            Some(i) => i,
        };
        let below_idx = top - 1;
        self.data.swap(top, below_idx);
        Ok(())
    }

    fn over(&mut self) -> Result<(), StackError> {
        let top = match self.top {
            Some(0) => return Err(StackError::NotEnoughElements),
            Some(i) => i,
            None => return Err(StackError::Empty),
        };
        let below = self.data[top - 1].clone();
        self.push(below.expect("expected entry 'below top' to be 'some' no 'none'"))
    }

    fn tuck(&mut self) -> Result<(), StackError> {
        let top = match self.top {
            Some(0) => return self.duplicate(),
            Some(i) => i,
            None => return Err(StackError::Empty),
        };

        if top + 1 >= N {
            return Err(StackError::Overflow);
        }

        let top_entry = self.data[top].expect("expected top to be Some");

        // Scary needs testing since indexing stuff and i am bad at that
        for i in (1..=top).rev() {
            self.data[i + 1] = self.data[i];
        }

        self.data[top - 1] = Some(top_entry);

        self.top = Some(top + 1);

        Ok(())
    }
}
// #endregion Stack

// #region StackError
#[derive(Debug)]
pub enum StackError {
    Overflow,
    Underflow,
    Empty,
    NotEnoughElements,
    Invalid(&'static str),
    InvalidStartIndex(usize),
    InvalidEndIndex(usize),
    InvalidRange(usize, usize, usize),
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackError::Overflow => write!(f, "Stack Overflow"),
            StackError::Underflow => write!(f, "Stack Underflow"),
            StackError::Invalid(msg) => write!(f, "Invalid stack state: {}", msg),
            StackError::Empty => write!(f, "Stack is empty"),
            StackError::NotEnoughElements => write!(f, "Operation requires more Arguments"),
            StackError::InvalidRange(start, end, top) => write!(
                f,
                "The Range from {} to {} is invalid for stack with top {}",
                start, end, top
            ),
            StackError::InvalidStartIndex(index) => {
                write!(f, "Lower bound index {} is out of range", index)
            }

            StackError::InvalidEndIndex(index) => {
                write!(f, "Upper bound index {} is out of range", index)
            }
        }
    }
}
// #endregion StackError
