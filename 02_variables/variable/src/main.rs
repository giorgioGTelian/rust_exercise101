fn main() {
    let x = 5; // Declare a variable `x` with value 5
    println!("The value of x is: {}", x); // Print the value of `x`
    
    // Uncommenting the next line will cause a compile-time error because `x` is immutable
    // x = 6; 

    let mut y = 10; // Declare a mutable variable `y` with value 10
    println!("The initial value of y is: {}", y); // Print the initial value of `y`
    
    y = 15; // Change the value of `y`
    println!("The updated value of y is: {}", y); // Print the updated value of `y`
}
