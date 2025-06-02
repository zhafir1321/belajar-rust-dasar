fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello, test!");
}

#[test]
fn hello_test_2() {
    println!("Hello, test 2!");
}

#[test]
fn test_variable() {
    // Variables in Rust are immutable by default, meaning they cannot be changed after declaration
    let name = "Zhafir Rasyid Muhammad Hafidz";
    println!("Hello, {}!", name);
}

#[test]
fn test_mutable() {
    // Mutable variables can be changed after they are declared
    let mut name = "Zhafir Rasyid Muhammad Hafidz";
    println!("Hello, {}!", name);

    name = "Zhafir Hafidz";

    println!("Hello, {}!", name);
}

#[test]
fn static_typing() {
    // Static typing means the type of a variable is known at compile time
    let mut name = "Zhafir Rasyid Muhammad Hafidz";
    println!("Hello, {}!", name);

    // name = 123; // This will cause a compile-time error because `name` is inferred as a string type
    name = "Zhafir Hafidz"; // This is valid as it's still a string
    println!("Hello, {}!", name);
}

#[test]
fn test_shadowing() {
    // Shadowing allows you to reuse the same variable name with a different type or value
    let name = "Zhafir Rasyid Muhammad Hafidz";
    println!("Hello, {}!", name);

    // Shadowing the variable with a new value
    let name = 123; // This is valid because it shadows the previous `name` variable
    println!("Hello, {}!", name);
}

#[test]
fn explicit() {
    let age: i32 = 25; // Explicitly declaring the type of the variable
    // i8, i16, i32, i64, i128, isize are signed integers
    // u8, u16, u32, u64, u128, usize are unsigned integers
    // f32, f64 are floating point numbers
    // isize and usize are pointer-sized integers, which depend on the architecture (32-bit or 64-bit)
    println!("Age: {}", age);
}

#[test]
fn number_conversion() {
    let a: i8 = 10; // i8 is an 8-bit signed integer

    println!("Value of a: {}", a);

    let b: i16 = a as i16; // Converting i8 to i16

    println!("Value of b: {}", b);

    let c: i32 = b as i32; // Converting i16 to i32
    println!("Value of c: {}", c);

    let d: i64 = 1000000000; // i64 is a 64-bit signed integer
    println!("Value of d: {}", d);

    let e: i8 = d as i8; // Converting i64 to i8 (this will truncate the value if it exceeds the range of i8)
    println!("Value of e: {}", e); // This may not give the expected result due to truncation
}

#[test]
fn numeric_operators() {
    let a: i32 = 10;
    let b: i32 = 20;

    let sum = a + b; // Addition
    let difference = b - a; // Subtraction
    let product = a * b; // Multiplication
    let quotient = b / a; // Division
    let remainder = b % a; // Modulus

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);
}
