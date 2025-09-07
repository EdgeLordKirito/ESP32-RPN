use crate::stack::*;
use esp_println::println;

#[cfg(feature = "stack_tests")]
pub fn run() {
    println!("Entered stack_test submodule");
    new();
    with();
    with_overflow();
    as_slice_empty();
    as_slice_filled();
    iter_empty();
    iter_filled();
    iter_rev_empty();
    iter_rev_filled();
    capacity();
    len();
    push();
    pop();
    peek();
    peek_range();
    empty();
    full();
    clear();
    fill();
    duplicate();
    delete_at();
    swap();
    over();
    tuck();
}

/// Test creating an empty stack with `new`
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn new() -> bool {
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
pub fn with() -> bool {
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
pub fn with_overflow() -> bool {
    let result = Stack::<i32, 2>::with([1, 2, 3]).is_err();
    if result {
        println!("test_stack_with_overflow passed");
    } else {
        println!("test_stack_with_overflow FAILED");
    }
    result
}

/// Test that `as_slice` returns an empty slice when the stack is empty
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn as_slice_empty() -> bool {
    let stack: Stack<i32, 4> = Stack::new();
    let slice = stack.as_slice();
    let result = slice.is_empty();

    if result {
        println!("test_stack_as_slice_empty passed");
    } else {
        println!("test_stack_as_slice_empty FAILED");
    }
    result
}

/// Test that `as_slice` returns the correct slice when the stack has elements
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn as_slice_filled() -> bool {
    let result;

    if let Ok(filled) = Stack::<i32, 3>::with([1, 2, 3]) {
        let slice = filled.as_slice();
        result =
            slice.len() == 3 && slice[0] == Some(1) && slice[1] == Some(2) && slice[2] == Some(3);
    } else {
        result = false;
    }

    if result {
        println!("test_stack_as_slice_filled passed");
    } else {
        println!("test_stack_as_slice_filled FAILED");
    }
    result
}

/// Test that iter() returns an empty iterator when the stack is empty
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn iter_empty() -> bool {
    let stack: Stack<i32, 4> = Stack::new();
    let mut result = true;

    // Forward iteration
    for _ in stack.iter() {
        result = false; // should not iterate any element
    }

    if result {
        println!("test_stack_iter_empty passed");
    } else {
        println!("test_stack_iter_empty FAILED");
    }

    result
}

/// Test that iter() returns elements in correct order for a filled stack
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn iter_filled() -> bool {
    let mut result = true;
    if let Ok(stack) = Stack::<i32, 3>::with([1, 2, 3]) {
        let expected = [1, 2, 3];
        let mut idx = 0;
        for val in stack.iter() {
            if val != expected[idx] {
                result = false;
                break;
            }
            idx += 1;
        }
    } else {
        result = false;
    }

    if result {
        println!("test_stack_iter_filled passed");
    } else {
        println!("test_stack_iter_filled FAILED");
    }
    result
}

/// Test that iter().rev() returns an empty iterator when the stack is empty
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn iter_rev_empty() -> bool {
    let stack: Stack<i32, 4> = Stack::new();
    let mut result = true;

    for _ in stack.iter().rev() {
        result = false; // should not iterate any element
    }

    if result {
        println!("test_stack_iter_rev_empty passed");
    } else {
        println!("test_stack_iter_rev_empty FAILED");
    }
    result
}

/// Test that iter().rev() returns elements in correct order for a filled stack
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn iter_rev_filled() -> bool {
    let mut result = true;
    if let Ok(stack) = Stack::<i32, 3>::with([1, 2, 3]) {
        let expected = [3, 2, 1];
        let mut idx = 0;
        for val in stack.iter().rev() {
            if val != expected[idx] {
                result = false;
                break;
            }
            idx += 1;
        }
    } else {
        result = false;
    }

    if result {
        println!("test_stack_iter_rev_filled passed");
    } else {
        println!("test_stack_iter_rev_filled FAILED");
    }
    result
}

/// Test capacity returns the const size
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn capacity() -> bool {
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
pub fn len() -> bool {
    let stack = Stack::<i32, 5>::with([10, 20]).unwrap();
    let result = stack.len() == 2;
    if result {
        println!("test_stack_len passed");
    } else {
        println!("test_stack_len FAILED");
    }
    result
}

/// Test that `push` adds elements and increases the stack length
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn push() -> bool {
    let mut stack: Stack<i32, 3> = Stack::new();
    let mut result = true;

    // Push 1, 2, 3
    result &= stack.push(1).is_ok();
    result &= stack.push(2).is_ok();
    result &= stack.push(3).is_ok();

    // Pushing beyond capacity should fail
    result &= matches!(stack.push(4), Err(StackError::Overflow));

    if result {
        println!("test_stack_push passed");
    } else {
        println!("test_stack_push FAILED");
    }
    result
}

/// Test that `pop` removes elements correctly
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn pop() -> bool {
    let mut stack: Stack<i32, 3> = Stack::new();
    let mut result = true;

    // Pop on empty stack should fail
    result &= matches!(stack.pop(), Err(StackError::Underflow));

    stack.push(10).unwrap();
    stack.push(20).unwrap();

    result &= stack.pop() == Ok(20);
    result &= stack.pop() == Ok(10);

    // Now empty again
    result &= matches!(stack.pop(), Err(StackError::Underflow));

    if result {
        println!("test_stack_pop passed");
    } else {
        println!("test_stack_pop FAILED");
    }
    result
}

/// Test that `peek` returns reference to top element without removing it
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn peek() -> bool {
    let mut stack: Stack<i32, 3> = Stack::new();
    let mut result = true;

    // Peek on empty stack should return None
    result &= stack.peek().is_none();

    stack.push(5).unwrap();
    stack.push(7).unwrap();

    // Peek should return 7
    result &= stack.peek() == Some(&7);
    // Length should not change
    result &= stack.len() == 2;

    if result {
        println!("test_stack_peek passed");
    } else {
        println!("test_stack_peek FAILED");
    }
    result
}

/// Test `peek_range` returns correct slices and errors for invalid ranges
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn peek_range() -> bool {
    let mut result = true;
    let stack: Stack<i32, 5> = Stack::with([1, 2, 3]).unwrap();

    // Normal ranges
    result &= stack.peek_range(0..2) == Ok(&[Some(1), Some(2)][..]);
    result &= stack.peek_range(0..=2) == Ok(&[Some(1), Some(2), Some(3)][..]);

    // Full range
    result &= stack.peek_range(..) == Ok(&[Some(1), Some(2), Some(3)][..]);

    // Empty stack should error
    let empty: Stack<i32, 3> = Stack::new();
    result &= matches!(empty.peek_range(..), Err(StackError::Empty));

    // Invalid range
    result &= matches!(
        stack.peek_range(2..5),
        Err(StackError::InvalidRange(_, _, _))
    );

    if result {
        println!("test_stack_peek_range passed");
    } else {
        println!("test_stack_peek_range FAILED");
    }
    result
}

/// Test `empty` returns true/false correctly
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn empty() -> bool {
    let mut result = true;
    let mut stack: Stack<i32, 3> = Stack::new();
    result &= stack.empty();

    stack.push(1).unwrap();
    result &= !stack.empty();

    if result {
        println!("test_stack_empty passed");
    } else {
        println!("test_stack_empty FAILED");
    }
    result
}

/// Test `full` returns true/false correctly
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn full() -> bool {
    let mut result = true;
    let mut stack: Stack<i32, 2> = Stack::new();
    result &= !stack.full();

    stack.push(1).unwrap();
    result &= !stack.full();

    stack.push(2).unwrap();
    result &= stack.full();

    if result {
        println!("test_stack_full passed");
    } else {
        println!("test_stack_full FAILED");
    }
    result
}

/// Test `clear` empties the stack
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn clear() -> bool {
    let mut stack: Stack<i32, 3> = Stack::with([1, 2]).unwrap();
    stack.clear();

    let result = stack.empty() && stack.len() == 0;

    if result {
        println!("test_stack_clear passed");
    } else {
        println!("test_stack_clear FAILED");
    }
    result
}

/// Test `fill` fills the stack to capacity with given value
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn fill() -> bool {
    let mut stack: Stack<i32, 3> = Stack::new();
    stack.fill(7);

    let slice = stack.as_slice();
    let result = slice == &[Some(7), Some(7), Some(7)] && stack.full();

    if result {
        println!("test_stack_fill passed");
    } else {
        println!("test_stack_fill FAILED");
    }
    result
}

/// Test `duplicate` copies the top element
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn duplicate() -> bool {
    let mut stack: Stack<i32, 3> = Stack::<i32, 3>::with([10]).unwrap();
    let result = stack.duplicate().is_ok()
        && stack.len() == 2
        && stack.as_slice()[0] == Some(10)
        && stack.as_slice()[1] == Some(10);

    if result {
        println!("test_stack_duplicate passed");
    } else {
        println!("test_stack_duplicate FAILED");
    }
    result
}

/// Test `delete_at` removes element and shifts stack
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn delete_at() -> bool {
    let mut stack: Stack<i32, 3> = Stack::<i32, 3>::with([1, 2, 3]).unwrap();
    let deleted = stack.delete_at(1);
    let result = deleted == Ok(2) && stack.len() == 2 && stack.as_slice() == &[Some(1), Some(3)];

    if result {
        println!("test_stack_delete_at passed");
    } else {
        println!("test_stack_delete_at FAILED");
    }
    result
}

/// Test `swap` swaps the top two elements
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn swap() -> bool {
    let mut stack: Stack<i32, 3> = Stack::<i32, 3>::with([5, 10]).unwrap();
    let result =
        stack.swap().is_ok() && stack.as_slice()[0] == Some(10) && stack.as_slice()[1] == Some(5);

    if result {
        println!("test_stack_swap passed");
    } else {
        println!("test_stack_swap FAILED");
    }
    result
}

/// Test `over` pushes a copy of the second element from the top
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn over() -> bool {
    let mut stack: Stack<i32, 3> = Stack::<i32, 3>::with([1, 2]).unwrap();
    let result = stack.over().is_ok()
        && stack.len() == 3
        && stack.as_slice() == &[Some(1), Some(2), Some(1)];

    if result {
        println!("test_stack_over passed");
    } else {
        println!("test_stack_over FAILED");
    }
    result
}

/// Test `tuck` inserts a copy of the top element below the second element
#[inline(never)]
#[cfg(feature = "stack_tests")]
pub fn tuck() -> bool {
    let mut stack: Stack<i32, 4> = Stack::<i32, 4>::with([1, 2, 3]).unwrap();
    let result = stack.tuck().is_ok()
        && stack.len() == 4
        && stack.as_slice() == &[Some(1), Some(3), Some(2), Some(3)];

    if result {
        println!("test_stack_tuck passed");
    } else {
        println!("test_stack_tuck FAILED");
    }
    result
}
