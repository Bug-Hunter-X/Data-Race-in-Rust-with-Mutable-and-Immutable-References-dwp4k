fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y = 10; // Modify x through y
    println!("x = {}", x); // Prints: x = 10
    println!("z = {}", *z); // This is now a data race!!!
}