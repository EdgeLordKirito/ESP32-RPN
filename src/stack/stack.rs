use core::fmt;

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

    pub fn push(&mut self, entry: StackEntry) -> Result<(), StackError> {
        let next = match self.top {
            None => 0,
            Some(i) if (i as usize) + 1 <= STACK_CAPACITY => i + 1,
            Some(_) => return Err(StackError::Overflow),
        };
        self.data[next as usize] = Some(entry);
        self.top = Some(next);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<StackEntry, StackError> {
        match self.top {
            None => Err(StackError::Underflow),
            Some(i) => {
                let val = self.data[i as usize].take().ok_or(StackError::Underflow)?;

                self.top = if i == 0 { None } else { Some(i - 1) };
                Ok(val)
            }
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
            Some(index) => index as usize,
        }
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
