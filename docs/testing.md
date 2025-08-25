# Testing Framework Guide

This project uses a custom `esp_test` module as a lightweight test framework.  
Tests are included conditionally using Cargo features, which allows running test code on the ESP32 without affecting normal firmware builds.
Throughout this guide, `<name>` is used as a placeholder for the specific test name you are adding (e.g. `stack`).

---

## Adding a New Submodule Test

Follow these steps to add a new test target:

### 1. Add a feature in `Cargo.toml`
Each test module must be tied to a Cargo feature.  
The feature **must depend on** the base `test` feature.

Example:
```toml
[features]
test = []
<name>_tests = ["test"]   # depends on test
```

### 2. Declare the test module in `esp_test/mod.rs`
Inside the `esp_test` module, add a conditional declaration.
Always use a feature with the `_tests` suffix to signal it is a test.

Example:
```rust
#[cfg(feature = "<name>_tests")]
pub mod <name>;
```


### 3. Create the test file
In the `esp_test` directory, create a file named `<name>.rs`.
This file should contain a `run` function that serves as the entry point for the tests.

Example:
```rust
pub fn run() {
    println!("Running <name> tests...");
    // Add test code here
}
```

### 4. Call the module from the entry point
Open `esp_test/entry.rs` and update the `run` function to call your test module:

```rust
pub fn run() {
    println!("Running test framework...");

    #[cfg(feature = "<name>_tests")]
    {
        use super::<name>;
        <name>::run();
    }
}