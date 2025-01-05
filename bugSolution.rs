fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y += 1; // Modifying x through y is allowed
    } // y goes out of scope here

    let z = &x;  // z is an immutable reference to x
    println!("x = {}", x); // Prints x = 6
    println!("z = {}", z); // Prints z = 6
} 