# Data Race in Rust with Mutable and Immutable References

This repository demonstrates a data race in Rust that occurs when multiple references to the same data exist without synchronization.  The code showcases how a mutable reference (`&mut`) and an immutable reference (`&`) to the same variable can lead to undefined behavior if the mutable reference modifies the data while the immutable reference is still active. 

## How to Reproduce

1. Clone the repository.
2. Navigate to the project directory.
3. Run the code using `cargo run`. 

The output will be unpredictable and may vary between runs, demonstrating the data race.

## Solution

The solution involves proper synchronization mechanisms to prevent data races.  The `bugSolution.rs` file demonstrates how to use a mutex or other methods to safely coordinate access to shared data.