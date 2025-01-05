# Rust Mutable and Immutable References Error

This example showcases a common pitfall in Rust programming: the interaction between mutable and immutable references.  The code attempts to create both mutable and immutable references to the same variable, leading to a compile-time error.

The solution demonstrates how to correctly manage these references to avoid the error, ensuring data safety and preventing race conditions. 

The `bug.rs` file contains the code demonstrating the error, while `bugSolution.rs` presents a corrected version.