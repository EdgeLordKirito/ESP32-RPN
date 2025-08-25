use esp_println::println;
pub fn run() {
    println!("Running test framework");
    #[cfg(feature = "stack_tests")]
    {
        use super::stack;
        stack::run();
    }
}
