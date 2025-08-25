pub const STACK_CAPACITY: usize = 256;

#[derive(Clone, Copy, Debug)]
pub enum StackEntry {
    Dummy, // Placeholder for actual stack entry types
    Integer(i32),
    Decimal(f32),
}
