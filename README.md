# Dangling Pointer Bug in Rust
This example demonstrates a common error in Rust: creating a dangling pointer.  A dangling pointer is a pointer that points to memory that has been freed or deallocated. Accessing a dangling pointer leads to undefined behavior, often resulting in crashes or unpredictable results.

The `bug.rs` file contains the buggy code, while `bugSolution.rs` provides a corrected version.

This is a critical issue that requires careful attention to memory management in Rust, especially when working with raw pointers.