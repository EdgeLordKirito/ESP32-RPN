use crate::stack::AdvancedStackOps;

use super::consts::STACK_CAPACITY;
use super::traits::StackOps;
use core::{fmt, ops::RangeBounds};

// #region Stack
#[derive(Debug)]
pub struct Stack {
    data: [Option<StackEntry>; STACK_CAPACITY],
    top: Option<u8>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            data: [None; STACK_CAPACITY],
            top: None,
        }
    }

    pub fn with<const N: usize>(entries: [StackEntry; N]) -> Result<Self, StackError> {
        if N > STACK_CAPACITY {
            return Err(StackError::Overflow);
        }

        let mut data: [Option<StackEntry>; STACK_CAPACITY] = [None; STACK_CAPACITY];
        for (i, entry) in entries.into_iter().enumerate() {
            data[i] = Some(entry);
        }
        let top = if N == 0 { None } else { Some((N - 1) as u8) };

        Ok(Stack { data, top })
    }

    pub fn capacity(&self) -> usize {
        STACK_CAPACITY
    }

    pub fn len(&self) -> usize {
        match self.top {
            None => 0,
            Some(i) => (i as usize) + 1,
        }
    }
}

impl StackOps<StackEntry, StackError> for Stack {
    fn push(&mut self, entry: StackEntry) -> Result<(), StackError> {
        let next = match self.top {
            None => 0,
            Some(i) if (i as usize) + 1 <= STACK_CAPACITY => i + 1,
            Some(_) => return Err(StackError::Overflow),
        };
        self.data[next as usize] = Some(entry);
        self.top = Some(next);
        Ok(())
    }

    fn pop(&mut self) -> Result<StackEntry, StackError> {
        match self.top {
            None => Err(StackError::Underflow),
            Some(i) => {
                let val = self.data[i as usize].take().ok_or(StackError::Underflow)?;

                self.top = if i == 0 { None } else { Some(i - 1) };
                Ok(val)
            }
        }
    }

    fn peek(&self) -> Option<&StackEntry> {
        match self.top {
            None => None,
            Some(i) => self.data[i as usize].as_ref(),
        }
    }

    fn peek_range<R>(&self, range: R) -> Result<&[Option<StackEntry>], StackError>
    where
        R: RangeBounds<usize>,
    {
        use core::ops::Bound;

        let start = match range.start_bound() {
            Bound::Included(&i) => try_usize_to_u8(i, BoundPosition::Start)?,
            Bound::Excluded(&i) => increment_checked_up_to_u8(i, BoundPosition::Start)?,
            Bound::Unbounded => 0,
        };

        let top_index = match self.top {
            Some(i) => i as usize,
            None => return Err(StackError::Empty),
        };

        let end = match range.end_bound() {
            Bound::Included(&i) => increment_checked_up_to_u8(i, BoundPosition::End)?,
            Bound::Excluded(&i) => try_usize_to_u8(i, BoundPosition::End)?,
            Bound::Unbounded => increment_checked_up_to_u8(top_index, BoundPosition::End)?,
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
        self.len() == STACK_CAPACITY
    }

    fn clear(&mut self) {
        for slot in self.data.iter_mut() {
            *slot = None;
        }
        self.top = None;
    }

    fn fill(&mut self, item: StackEntry) {
        for slot in self.data.iter_mut() {
            *slot = Some(item.clone());
        }
        self.top = Some(255);
    }
}

impl AdvancedStackOps<StackEntry, StackError> for Stack {
    fn duplicate(&mut self) -> Result<(), StackError> {
        match self.top {
            None => Err(StackError::Empty),
            Some(i) => {
                if (i as usize) + 1 >= STACK_CAPACITY {
                    return Err(StackError::Overflow);
                }
                match self.data[i as usize].as_ref() {
                    Some(entry) => self.push(entry.clone()),
                    None => Err(StackError::Invalid("Corrupted stack: no Entry at top")),
                }
            }
        }
    }

    fn delete_at(&mut self, index: u8) -> Result<StackEntry, StackError> {
        let idx = index as usize;

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
                    Some((top_idx - 1) as u8)
                };
                Ok(entry)
            }
            None => Err(StackError::Empty),
        }
    }

    fn swap(&mut self) -> Result<(), StackError> {
        let top = match self.top {
            Some(i) => i,
            None => return Err(StackError::Empty),
        };
        let top_idx = top as usize;
        let below_idx = top_idx - 1;
        self.data.swap(top_idx, below_idx);
        Ok(())
    }

    // Push a copy of the second element from the top onto the stack.
    fn over(&mut self) -> Result<(), StackError> {
        let top = match self.top {
            Some(i) => i,
            None => 0,
        };
        if top == 0 {
            return Err(StackError::Empty);
        }
        let below = self.data[(top - 1) as usize].clone();
        self.push(below.expect("expected entry 'below top' to be 'some' no 'none'"))
    }

    // Insert a copy of the top element just below the second element or first if there is no second element.
    fn tuck(&mut self) -> Result<(), StackError> {
        let top = match self.top {
            Some(i) => i,
            None => return Err(StackError::Empty),
        };

        if top == 0 {
            // No second element, just duplicate the top
            return self.duplicate();
        }

        let top_idx = top as usize;
        let top_entry = self.data[top_idx].clone().expect("expected top to be Some");

        // Scary needs testing since indexing stuff and i am bad at that
        for i in (top_idx..self.data.len() - 1).rev() {
            self.data[i + 1] = self.data[i].take();
        }

        self.data[top_idx - 1] = Some(top_entry);

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
    Invalid(&'static str),
    InvalidRange(usize, usize, usize),
    InvalidStartIndex(usize),
    InvalidEndIndex(usize),
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackError::Overflow => write!(f, "Stack Overflow"),
            StackError::Underflow => write!(f, "Stack Underflow"),
            StackError::Invalid(msg) => write!(f, "Invalid stack state: {}", msg),
            StackError::Empty => write!(f, "Stack is empty"),
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

#[derive(Clone, Copy, Debug)]
pub enum StackEntry {
    Dummy, // Placeholder for actual stack entry types
}

// #region helper_functions
#[derive(Clone, Copy, Debug)]
enum BoundPosition {
    Start,
    End,
}

fn try_usize_to_u8(val: usize, bound_pos: BoundPosition) -> Result<usize, StackError> {
    u8::try_from(val)
        .map(|v| v as usize)
        .map_err(|_| to_invalid_index_err(bound_pos, val))
}

fn increment_checked_up_to_u8(val: usize, bound_pos: BoundPosition) -> Result<usize, StackError> {
    let inc = val
        .checked_add(1)
        .ok_or(to_invalid_index_err(bound_pos, val))?;
    try_usize_to_u8(inc, bound_pos)
}

fn to_invalid_index_err(pos: BoundPosition, index: usize) -> StackError {
    match pos {
        BoundPosition::Start => StackError::InvalidStartIndex(index),
        BoundPosition::End => StackError::InvalidEndIndex(index),
    }
}
// #endregion helper_functions
