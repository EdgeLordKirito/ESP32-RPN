#[cfg(feature = "stack_tests")]
use crate::stack::Stack;
use esp_println::println;

#[cfg(feature = "stack_tests")]
pub fn run() {
    println!("Entered stack_test submodule");
    test_stack_new();
    test_stack_with();
    test_stack_with_overflow();
    test_stack_capacity();
    test_stack_len();
}

/// Test creating an empty stack with `new`
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn test_stack_new() -> bool {
    let stack: Stack<i32, 4> = Stack::new();
    let result = stack.len() == 0 && stack.capacity() == 4;
    if result {
        println!("test_stack_new passed");
    } else {
        println!("test_stack_new FAILED");
    }
    result
}

/// Test creating a stack with entries using `with`
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn test_stack_with() -> bool {
    let stack = Stack::<i32, 4>::with([1, 2, 3]).unwrap();
    let result = stack.len() == 3 && stack.capacity() == 4;
    if result {
        println!("test_stack_with passed");
    } else {
        println!("test_stack_with FAILED");
    }
    result
}

/// Test `with` returns error when too many entries are provided
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn test_stack_with_overflow() -> bool {
    let result = Stack::<i32, 2>::with([1, 2, 3]).is_err();
    if result {
        println!("test_stack_with_overflow passed");
    } else {
        println!("test_stack_with_overflow FAILED");
    }
    result
}

/// Test capacity returns the const size
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn test_stack_capacity() -> bool {
    let stack: Stack<i32, 8> = Stack::new();
    let result = stack.capacity() == 8;
    if result {
        println!("test_stack_capacity passed");
    } else {
        println!("test_stack_capacity FAILED");
    }
    result
}

/// Test len matches the number of inserted entries
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn test_stack_len() -> bool {
    let stack = Stack::<i32, 5>::with([10, 20]).unwrap();
    let result = stack.len() == 2;
    if result {
        println!("test_stack_len passed");
    } else {
        println!("test_stack_len FAILED");
    }
    result
}
