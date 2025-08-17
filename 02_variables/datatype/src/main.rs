fn main() {
    let greeting = "Hello, world!";
    println!("{}", greeting);

    let farewell = "Goodbye, world!";
    println!("{}", farewell);

    let x: i32 = 5;
    let y: f64 = 3.14;
    let z: bool = true;

    println!("x: {}, y: {}, z: {}", x, y, z);

    let sum = x + y as i32;
    println!("Sum of x and y: {}", sum);

    let product = x * y as i32;
    println!("Product of x and y: {}", product);

    let is_equal = x == z as i32;
    println!("Is x equal to z? {}", is_equal);

    let is_greater = x > y as i32;
    println!("Is x greater than y? {}", is_greater);

    let is_less = x < y as i32;
    println!("Is x less than y? {}", is_less);

    let is_not_equal = x != z as i32;
    println!("Is x not equal to z? {}", is_not_equal);

    let is_farewell = farewell == "Goodbye, world!";
    println!("Is farewell equal to 'Goodbye, world!'? {}", is_farewell);

    let is_greeting = greeting == "Hello, world!";
    println!("Is greeting equal to 'Hello, world!'? {}", is_greeting);

    const MAX_LIMIT: i32 = 100;
    println!("Max limit is: {}", MAX_LIMIT);

    const MIN_LIMIT: i32 = 0;
    println!("Min limit is: {}", MIN_LIMIT);

    let is_within_limits = x > MIN_LIMIT && x < MAX_LIMIT;
    println!("Is x within limits? {}", is_within_limits);

    let mut mutable_var = 10;
    println!("Mutable variable is: {}", mutable_var);

    mutable_var += 5;
    println!("Mutable variable after addition is: {}", mutable_var);

    mutable_var *= 2;
    println!("Mutable variable after multiplication is: {}", mutable_var);

    mutable_var -= 3;
    println!("Mutable variable after subtraction is: {}", mutable_var);

    mutable_var = 0;
    println!("Mutable variable after reset is: {}", mutable_var);

    mutable_var = 42;
    println!("Mutable variable after reassignment is: {}", mutable_var);

    mutable_var += 1;
    println!("Mutable variable after increment is: {}", mutable_var);

    // End of mutable variable demonstration

    let another_var = 30;
    println!("Another variable is: {}", another_var);

    // End of another variable demonstration

    // End of variable demonstration

    //data types
    let integer_var: i32 = 42;
    let float_var: f64 = 3.14;
    let boolean_var: bool = true;
    let char_var: char = 'A';
    let string_var: &str = "Hello, Rust!";

    println!("Integer variable is: {}", integer_var);
    println!("Float variable is: {}", float_var);
    println!("Boolean variable is: {}", boolean_var);
    println!("Character variable is: {}", char_var);
    println!("String variable is: {}", string_var);
}
