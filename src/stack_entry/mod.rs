use crate::limbo::Limbo;

#[derive(Clone, Copy, Debug)]
pub enum StackEntry {
    Integer(i32),
    Decimal(f32),
}

impl StackEntry {
    pub fn as_str<'a>(&self, buffer: &'a mut BufferType) -> &'a str {
        match (self, buffer) {
            (StackEntry::Integer(val), BufferType::Integer(buf)) => buf.format(*val),
            (StackEntry::Decimal(val), BufferType::Decimal(buf)) => buf.format(*val),
            // fallback if types don't match
            _ => panic!("mismatch"),
        }
    }

    pub fn from(limbo: Limbo) -> Self {
        todo!("implement this")
    }
}

pub enum BufferType {
    Integer(itoa::Buffer),
    Decimal(ryu::Buffer),
}

impl BufferType {
    /// Create a new integer buffer
    pub fn new_int() -> Self {
        BufferType::Integer(itoa::Buffer::new())
    }

    /// Create a new decimal buffer
    pub fn new_dec() -> Self {
        BufferType::Decimal(ryu::Buffer::new())
    }

    /// Create a buffer that matches the type of the given StackEntry
    pub fn new_for(entry: &StackEntry) -> Self {
        match entry {
            StackEntry::Integer(_) => BufferType::Integer(itoa::Buffer::new()),
            StackEntry::Decimal(_) => BufferType::Decimal(ryu::Buffer::new()),
        }
    }
}
