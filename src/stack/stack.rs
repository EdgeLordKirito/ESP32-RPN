use core::{fmt, ops::RangeBounds};

const STACK_CAPACITY: usize = 256;

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

    pub fn duplicate(&mut self) -> Result<(), StackError> {
        match self.top {
            None => Err(StackError::Invalid("Empty")),
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
    fn peek_range<R>(&self, range: R) -> Result<&[StackEntry], StackError>
    where
        R: RangeBounds<usize>,
    {
        todo!("need more info about Ranges")
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

#[derive(Clone, Copy, Debug)]
pub enum StackEntry {}

#[derive(Debug)]
pub enum StackError {
    Overflow,
    Underflow,
    Invalid(&'static str),
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackError::Overflow => write!(f, "Stack Overflow"),
            StackError::Underflow => write!(f, "Stack Underflow"),
            StackError::Invalid(msg) => write!(f, "Invalid stack state: {}", msg),
        }
    }
}

pub trait StackOps<T, E> {
    fn push(&mut self, item: T) -> Result<(), E>;
    fn pop(&mut self) -> Result<T, E>;
    fn peek(&self) -> Option<&T>;
    fn peek_range<R>(&self, range: R) -> Result<&[T], E>
    where
        R: RangeBounds<usize>;
    fn empty(&self) -> bool;
    fn full(&self) -> bool;
    fn clear(&mut self);
    fn fill(&mut self, item: T);
}
