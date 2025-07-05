mod first; // Importing the first module
mod model; // Importing the model module
mod second; // Importing the second module
mod third; // Importing the third module

use model::User; // Importing the User struct from the model module

#[test]
fn test_user() {
    let user = User {
        name: String::from("Zhafir"),
        age: 25,
        email: String::from("zahfir1000@mail.com"),
    };

    user.say_hello("Budi"); // Calling the say_hello method on the user instance
}

use first::say_hello;
use second::say_hello as say_hello_second; // Renaming the function to avoid conflict

#[test]
fn test_modules() {
    // Calling the say_hello function from the first module
    say_hello();
    // Calling the say_hello function from the second module
    say_hello_second();
}

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

#[test]
fn test_tuple() {
    // Tuples are fixed-size collections of values of different types
    let data: (i32, f32, bool) = (42, 3.14, true); // A tuple containing an integer, a float, and a boolean
    println!("Tuple data: {:?}", data);

    let a = data.0; // Accessing the first element of the tuple
    let b = data.1; // Accessing the second element of the tuple
    let c = data.2; // Accessing the third element of the tuple
    println!("First element: {}", a);
    println!("Second element: {}", b);
    println!("Third element: {}", c);

    let (x, y, z) = data; // Destructuring the tuple into individual variables
    println!("Destructured values: x = {}, y = {}, z = {}", x, y, z);
}

#[test]
fn test_mutable_tuple() {
    // Tuples can contain mutable elements, but the tuple itself must be mutable to change its contents
    let mut data: (i32, f32, bool) = (42, 3.14, true);
    println!("Original tuple: {:?}", data);

    data.0 = 100; // Changing the first element of the tuple
    data.1 = 2.71; // Changing the second element of the tuple
    data.2 = false; // Changing the third element of the tuple
    println!("Modified tuple: {:?}", data);
}

fn unit() {
    // The unit type `()` is used when a function does not return a value
    println!("This function returns nothing, it has the unit type.");
}

#[test]
fn test_unit() {
    // Calling the unit function
    unit();

    // The unit type is often used in functions that perform actions but do not return a value
    let result: () = unit(); // This is valid, but the result is not used
    println!("Result of unit function: {:?}", result);

    let test: () = (); // This is also valid, representing an empty value
    println!("Test variable with unit type: {:?}", test);
}

#[test]
fn test_array() {
    let array = [1, 2, 3, 4, 5]; // An array of integers
    println!("Array: {:?}", array);

    let array2: [i32; 5] = [10, 20, 30, 40, 50]; // An array with explicit type and size
    println!("Array2: {:?}", array2);

    // Accessing elements of the array
    let first_element = array[0]; // Accessing the first element
    let second_element = array[1]; // Accessing the second element
    println!("First element: {}", first_element);
    println!("Second element: {}", second_element);
}

#[test]
fn test_mutable_array() {
    let mut array = [1, 2, 3, 4, 5]; // A mutable array of integers
    println!("Original array: {:?}", array);

    // Modifying elements of the array
    array[0] = 10; // Changing the first element
    array[1] = 20; // Changing the second element
    println!("Modified array: {:?}", array);

    let length: usize = array.len(); // Getting the length of the array
    println!("Length of the array: {}", length);
}

#[test]
fn test_dimensional_array() {
    // A two-dimensional array (matrix) in Rust
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]]; // A 2x3 matrix
    println!("Matrix: {:?}", matrix);

    // Accessing elements of the two-dimensional array
    let first_row = matrix[0]; // Accessing the first row
    let second_row = matrix[1]; // Accessing the second row
    println!("First row: {:?}", first_row);
    println!("Second row: {:?}", second_row);

    let element = matrix[0][1]; // Accessing an element in the first row and second column
    println!("Element at (0, 1): {}", element);
}

const MAXIMUM: i32 = 100; // A constant value that cannot be changed
#[test]
fn test_constant() {
    // Constants in Rust are immutable and must have a type annotation
    println!("The maximum value is: {}", MAXIMUM);

    const MINIMUM: i32 = 0; // Another constant
    println!("The minimum value is: {}", MINIMUM);
}

#[test]
fn test_string() {
    let name: &str = "  Zhafir Rasyid Muhammad Hafidz  ";
    let trimmed_name = name.trim(); // Trimming whitespace from the string
    println!("Original name: '{}'", name);
    println!("Trimmed name: '{}'", trimmed_name);

    let mut username: &str = "Zhafir Hafidz"; // still in a memory location, but mutable
    username = "Zhafir Rasyid"; // "Zhafir Hafidz" still in a memory location, but now `username` points to a new string
    println!("Username: '{}'", username);
}

#[test]
fn test_string_type() {
    let mut name: String = String::from("Zhafir Rasyid Muhammad"); // A mutable string type
    println!("Name: '{}'", name);
    name.push_str(" Hafidz"); // Appending to the string
    println!("Updated Name: '{}'", name);

    let replaced = name.replace("Zhafir", "zhafir");
    println!("Replaced Name: '{}'", replaced);
}

#[test]
fn test_ownership_rules() {
    // Ownership rules in Rust ensure memory safety without a garbage collector
    // a tidak bisa diakses di sini, karena belum dideklarasikan
    let a = 1; // `a` is the owner of the value `1`, a bisa diakses di sini

    {
        // b tidak bisa diakses di sini, karena belum dideklarasikan
        let b = 5; // `b` is the owner of the value `5`, b bisa diakses di sini
        println!("Value of a: {}", a); // a masih bisa diakses di sini
        println!("Value of b: {}", b); // b bisa diakses di sini
    } // b tidak bisa diakses di sini, karena sudah keluar dari scope, b dihapus dari memori

    println!("Value of a after b's scope: {}", a); // a masih bisa diakses di sini
} // a tidak bisa diakses di sini, karena sudah keluar dari scope, a dihapus dari memori

#[test]
fn test_ownership_transfer() {
    let s1 = String::from("Hello"); // `s1` owns the string "Hello"
    let s2 = s1; // Ownership of the string is transferred to `s2`, `s1` is no longer valid

    // println!("s1: {}", s1); // This would cause a compile-time error because `s1` is no longer valid
    println!("s2: {}", s2); // This is valid, `s2` owns the string now
    // println!("s1: {}", s1); // This will cause a compile-time error because `s1` is no longer valid
    let s3 = s2.clone(); // Cloning `s2` creates a new instance of the string, `s3` is now a separate owner of the string
    println!("s3: {}", s3); // This is valid, `s3` owns a clone of the string
}

#[test]
fn test_if_let_statement() {
    let value = 9;
    let result = if value > 10 {
        // Using an if-else statement to determine the result based on the value
        "Greater than 10"
    } else if value < 10 {
        "Less than 10"
    } else {
        "Equal to 10"
    };
    println!("Result: {}", result); // This will print "Less than 10" since value is 9
}

#[test]
fn test_loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;

        if counter > 10 {
            break;
        }

        println!("Counter: {}", counter); // This will print the counter value from 1 to 10
    }
}

#[test]
fn test_loop_let_expression() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter > 10 {
            break counter * 2; // This will return the value of counter multiplied by 2 when it breaks
        }
    };

    println!("Result of loop: {}", result); // This will print "Result of loop: 22" since counter will be 11 when it breaks
}

#[test]
fn test_loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer; // Breaks out of the outer loop
            }

            println!("{} * {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn test_while_loop() {
    let mut counter = 0;
    while counter < 10 {
        counter += 1;
        println!("Counter: {}", counter); // This will print the counter value from 1 to 10
    }
}

#[test]
fn test_array_iteration() {
    let array = ["alpha", "beta", "charlie", "delta"]; // An array of strings
    for element in array.iter() {
        println!("Element: {}", element); // This will print each element of the array
    }
}

#[test]
fn test_loop_range() {
    let range = 0..5;
    println!("Range: {:?}", range.start); // This will print the start of the range, which is 0
    println!("Range: {:?}", range.end); // This will print the end of the range, which is 5

    for i in range {
        println!("Value: {}", i); // This will print values from 0 to 4
    }

    let array = ["A", "B", "C", "D", "E"];
    for i in 0..array.len() {
        // Iterating over the indices of the array
        println!("Array element at index {}: {}", i, array[i]); // This will print each element of the array by index
    }
}

#[test]
fn test_loop_range_inclusive() {
    let range = 0..=5; // This is an inclusive range, meaning it includes the end value
    println!("Inclusive Range: {:?}", range.start()); // This will print the start of the range, which is 0
    println!("Inclusive Range: {:?}", range.end()); // This will print the end of the range, which is 5

    for i in range {
        println!("Value: {}", i); // This will print values from 0 to 5
    }
}

fn print_number(number: i32) {
    println!("number: {}", number);
}

fn hi(name: String) {
    println!("name : {}", name);
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name = String::from("Zhafir Rasyid Muhammad Hafidz"); // name owns the String data
    hi(name); // The `hi` function takes ownership of `name`, so it cannot be used after this point
    // println!("{}", name); // This line would cause a compile-time error because `name` has been moved to the `hi` function and is no longer valid here
}

fn full_name(first_name: String, last_name: String) -> (String, String, String) {
    // This function takes ownership of both first_name and last_name
    let full_name = format!("{} {}", first_name, last_name);

    (first_name, last_name, full_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("Zhafir");
    let last_name = String::from("Hafidz");

    let (first_name, last_name, full_name) = full_name(first_name, last_name); // Destructuring the tuple returned by full_name function

    println!("First Name: {}", first_name);
    println!("Last Name: {}", last_name);
    println!("Full Name: {}", full_name);
}

fn reference(first_name: &String, last_name: &String) -> String {
    // This function takes references to first_name and last_name
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_reference() {
    let first_name = String::from("Zhafir");
    let last_name = String::from("Hafidz");

    // Passing references to the reference function
    let full_name = reference(&first_name, &last_name);

    println!("First Name: {}", first_name);
    println!("Last Name: {}", last_name);
    println!("Full Name: {}", full_name);
}

fn mutable_reference(first_name: &mut String, last_name: &mut String) -> String {
    // &mut String allows modification of the original strings
    // This function takes mutable references to first_name and last_name
    first_name.push_str(" Rasyid");
    last_name.push_str(" Muhammad Hafidz");

    format!("{} {}", first_name, last_name)
}

#[test]
fn test_mutable_reference() {
    let mut first_name = String::from("Zhafir");
    let mut last_name = String::from("Hafidz");

    // Passing mutable references to the mutable_reference function
    let full_name = mutable_reference(&mut first_name, &mut last_name); // 

    println!("First Name: {}", first_name);
    println!("Last Name: {}", last_name);
    println!("Full Name: {}", full_name);
}

#[test]
fn test_slice_reference() {
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // index from 0 to 9
    let slice1: &[i32] = &array[..]; // .. refers to the entire array, creating a slice that includes all elements
    println!("Slice 1: {:?}", slice1);

    let slice2: &[i32] = &array[0..5]; // 0..5 is a range that refers to the first five elements of the array // (index 0 to 4)
    println!("Slice 2: {:?}", slice2);

    let slice3: &[i32] = &array[5..]; // 5.. refers to the elements from index 5 to the end of the array // (index 5 to 9)
    println!("Slice 3: {:?}", slice3);

    let slice4: &[i32] = &array[0..=5]; // 0..=5 is a range that includes the first five elements, but not the sixth // (index 0 to 5)
    println!("Slice 4: {:?}", slice4);

    let slice5: &[i32] = &array[..5]; // ..5 is a range that refers to the first five elements of the array // (index 0 to 4)
    println!("Slice 5: {:?}", slice5);

    let slice6: &[i32] = &array[..=5]; // ..=5 is a range that includes the first five elements, but not the sixth // (index 0 to 5)
    println!("Slice 6: {:?}", slice6);
}

#[test]
fn test_string_slice() {
    let full_name = String::from("Zhafir Rasyid Muhammad Hafidz");

    let first_name: &str = &full_name[0..6]; // Slicing the string to get the first name (index 0 to 5)
    let last_name: &str = &full_name[23..]; // Slicing the string to get the last name (index 23 to the end)
    println!("First Name: {}", first_name);
    println!("Last Name: {}", last_name);
}

// Structs in Rust are used to create custom data types
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

fn print_person(person: &Person) {
    // This function takes a reference to a Person struct
    println!("First Name: {}", person.first_name);
    println!("Last Name: {}", person.last_name);
    println!("Age: {}", person.age);
}

#[test]
fn test_struct() {
    let person: Person = Person {
        first_name: String::from("Zhafir"),
        last_name: String::from("Hafidz"),
        age: 25,
    };
    print_person(&person); // Passing a reference to the print_person function
    // println!("Person: {}", person); // This line would cause a compile-time error because `Person` does not implement the `Display` trait
}

#[test]
fn test_init_shorthand() {
    let first_name = String::from("Zhafir");
    let last_name = String::from("Hafidz");
    let age = 25;

    // Using struct initialization shorthand
    let person = Person {
        first_name,
        last_name,
        age,
    };

    print_person(&person); // Passing a reference to the print_person function

    let person2: Person = Person { ..person }; // Using struct update syntax to create a new instance of Person with the same values as `person`
    print_person(&person2); // Passing a reference to the print_person function

    let person3: Person = Person {
        // first_name: person.first_name.clone(), // cannot use `clone` of person because person.first_name owner changed to person2
        first_name: person2.first_name.clone(), // Using `clone` to create a new instance of first_name
        // last_name: person.last_name.clone(), // cannot use `clone` of person because person.last_name owner changed to person2
        last_name: person2.last_name.clone(), // Using `clone` to create a new instance of last_name
        ..person
    };
    print_person(&person3); // Passing a reference to the print_person function
}

struct GeoPoint(f64, f64);

impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        // associated function to create a new instance of GeoPoint
        GeoPoint(long, lat)
    }
}

#[test]
fn test_tuple_struct() {
    let geo_point = GeoPoint(37.7749, -122.4194); // Creating an instance of GeoPoint with latitude and longitude
    println!("GeoPoint: ({}, {})", geo_point.0, geo_point.1); // Accessing the fields of the tuple struct using index notation
}

struct Nothing; // Unit struct with no fields

#[test]
fn test_nothing() {
    let _nothing1: Nothing = Nothing; // Creating an instance of the unit struct
    let _nothing2: Nothing = Nothing {};
}

impl Person {
    fn say_hello(&self, name: &str) {
        // self is a reference to the instance of Person // &self means we are borrowing the instance, not taking ownership
        // This method takes a reference to self and a string slice as an argument
        println!(
            "Hello, {}! My name is {} {} and I am {} years old.",
            name, self.first_name, self.last_name, self.age
        );
    }
}

#[test]
fn test_method() {
    let person = Person {
        first_name: String::from("Rasyid"),
        last_name: String::from("Hafidz"),
        age: 25,
    };

    person.say_hello("Alice"); // Calling the say_hello method on the person instance
}

#[test]
fn test_associated_function() {
    let geo_point = GeoPoint::new(37.7749, -122.4194); // Calling the associated function to create a new instance of GeoPoint
    println!("GeoPoint: ({}, {})", geo_point.0, geo_point.1); // Accessing the fields of the tuple struct using index notation

    // let geo_point2 = geo_point.new(40.7128, -74.0060); // cause error because `new` is not an instance method, but an associated function
}

enum Level {
    Beginner,
    Intermediate,
    Advanced,
}

#[test]
fn test_enum() {
    let level: Level = Level::Intermediate; // Creating an instance of the Level enum

    match level {
        // match expression to handle different variants of the enum
        Level::Beginner => println!("You are a beginner!"),
        Level::Intermediate => println!("You are at an intermediate level!"),
        Level::Advanced => println!("You are an advanced user!"),
    }
}

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}

#[test]
fn test_payment() {
    let _payment1: Payment = Payment::CreditCard(String::from("1234-5678-9012-3456")); // Creating an instance of the Payment enum with CreditCard variant
    let _payment2: Payment =
        Payment::BankTransfer(String::from("Bank A"), String::from("1234567890")); // Creating an instance of the Payment enum with BankTransfer variant
    let _payment3: Payment = Payment::EWallet(String::from("BCA"), String::from("1234567890")); // Creating an instance of the Payment enum with EWallet variant
}

impl Payment {
    fn pay(&self, amount: u32) {
        // enum method to process payment
        match self {
            Payment::CreditCard(number) => {
                println!(
                    "Processing credit card payment of {} with card number: {}",
                    amount, number
                );
            }
            Payment::BankTransfer(bank, account) => {
                println!(
                    "Processing bank transfer payment of {} from bank: {} with account number: {}",
                    amount, bank, account
                );
            }
            Payment::EWallet(provider, account) => {
                println!(
                    "Processing e-wallet payment of {} from provider: {} with account number: {}",
                    amount, provider, account
                );
            }
        }
    }
}

#[test]
fn test_payment_method() {
    let _payment1: Payment = Payment::CreditCard(String::from("1234-5678-9012-3456")); // Creating an instance of the Payment enum with CreditCard variant
    let _payment2: Payment =
        Payment::BankTransfer(String::from("Bank A"), String::from("1234567890")); // Creating an instance of the Payment enum with BankTransfer variant
    let _payment3: Payment = Payment::EWallet(String::from("BCA"), String::from("1234567890")); // Creating an instance of the Payment enum with EWallet variant
    _payment1.pay(1000); // Calling the pay method on the CreditCard variant
    _payment2.pay(2000); // Calling the pay method on the BankTransfer variant
    _payment3.pay(3000); // Calling the pay method on the EWallet variant
}

#[test]
fn test_match_value() {
    let name = "Zhafir";

    match name {
        "Zhafir" => {
            println!("Hello, Zhafir!");
        }
        "Rasyid" => {
            println!("Hello, Rasyid!");
        }
        other => {
            println!("Hello, {}!", other); // This will match any other value not explicitly handled above
        }
    }

    match name {
        "Zhafir" | "Rasyid" => {
            println!("Hello, Bos!");
        }

        other => {
            println!("Hello, {}!", other); // This will match any other value not explicitly handled above
        }
    }
}

#[test]
fn test_match_range() {
    let value = 24;

    match value {
        75..=100 => {
            println!("Great!")
        }
        50..=74 => {
            println!("Good!")
        }
        25..=49 => {
            println!("Not bad!")
        }
        0..=24 => {
            println!("You can do better!")
        }
        _ => {
            println!("Invalid value!")
        }
    }
}

#[test]
fn test_struct_pattern() {
    let point = GeoPoint::new(0.0, 1.0);

    match point {
        GeoPoint(long, 0.0) => {
            println!("Point is on the x-axis at longitude: {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("Point is on the y-axis at latitude: {}", lat);
        }
        _ => {
            println!("Point is at coordinates: ({}, {})", point.0, point.1);
        }
    }

    let person = Person {
        first_name: String::from("Zhafir"),
        last_name: String::from("Hafidz"),
        age: 25,
    };

    match person {
        Person {
            first_name,
            last_name,
            ..
        } => {
            println!("Person: {} {}", first_name, last_name);
        }
    }
}

#[test]
fn test_ignoring() {
    let point = GeoPoint::new(0.0, 1.0);

    match point {
        GeoPoint(long, _) => {
            // Ignoring the latitude value
            println!("Point is on the x-axis at longitude: {}", long);
        }
        GeoPoint(_, lat) => {
            // Ignoring the longitude value
            println!("Point is on the y-axis at latitude: {}", lat);
        }
        _ => {
            println!("Point is at coordinates: ({}, {})", point.0, point.1);
        }
    }
}

#[test]
fn test_match_expression() {
    let value = 10;

    let result = match value {
        0 => "Zero",
        1..=5 => "One to Five",
        6..=10 => "Six to Ten",
        _ => "Greater than Ten",
    };

    println!("Result: {}", result); // This will print the matched result based on the value of `value`
}

type Age = u8; // Type alias for u8, making it easier to read and understand
type IdentityNumber = String; // Type alias for String, representing an identity number

struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age,
}

type Pelanggan = Customer; // Type alias for Customer, making it easier to read and understand
#[test]
fn test_customer() {
    let customer = Customer {
        id: String::from("1234567890"),
        name: String::from("Zhafir Rasyid Muhammad Hafidz"),
        age: 25,
    };

    // customer.id; // Accessing the id field of the Customer struct
    // customer.name; // Accessing the name field of the Customer struct
    // customer.age; // Accessing the age field of the Customer struct
    println!("Customer ID: {}", customer.id);
    println!("Customer Name: {}", customer.name);
    println!("Customer Age: {}", customer.age);
}

trait CanSayHello {
    fn hello(&self) -> String {
        String::from("Hello from CanSayHello trait!")
    }
    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

trait CanSayGoodBye {
    fn goodbye(&self) -> String {
        String::from("Goodbye from CanSayGoodbye trait!")
    }
    fn say_goodbye(&self) -> String;
    fn say_goodbye_to(&self, name: &str) -> String;
}

/*

*/

impl CanSayHello for Person {
    fn say_hello(&self) -> String {
        // Implementing the say_hello method for the Person struct
        format!("Hello, my name is {} {}!", self.first_name, self.last_name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        // Implementing the say_hello_to method for the Person struct
        format!(
            "Hello, {}! My name is {} {}.",
            name, self.first_name, self.last_name
        )
    }
}

impl CanSayGoodBye for Person {
    fn say_goodbye(&self) -> String {
        // Implementing the say_goodbye method for the Person struct
        format!("Goodbye from {} {}!", self.first_name, self.last_name)
    }

    fn say_goodbye_to(&self, name: &str) -> String {
        // Implementing the say_goodbye_to method for the Person struct
        format!(
            "Goodbye, {}! From {} {}.",
            name, self.first_name, self.last_name
        )
    }
}

#[test]
fn test_trait() {
    let person = Person {
        first_name: String::from("Zhafir"),
        last_name: String::from("Hafidz"),
        age: 25,
    };
    let result = person.say_hello_to("Budi");
    println!("{}", result); // This will print the greeting message from the say_hello method

    let result2 = person.hello();
    println!("{}", result2); // This will print the default message from the CanSayHello trait

    let result3 = person.say_goodbye_to("Budi");
    println!("{}", result3); // This will print the goodbye message from the say_goodbye_to method
    let result4 = person.goodbye();
    println!("{}", result4); // This will print the default message from the CanSayGoodBye trait
}

fn say_hello_trait(person: &impl CanSayHello) {
    // This function takes a reference to any type that implements the CanSayHello trait
    println!("{}", person.say_hello()); // Calling the say_hello method on the person instance
}

fn say_hello_goodbye_trait(value: &(impl CanSayHello + CanSayGoodBye)) {
    // This function takes a reference to any type that implements both CanSayHello and CanSayGoodBye traits
    println!("{}", value.say_hello()); // Calling the say_hello method on the value instance
    println!("{}", value.say_goodbye()); // Calling the say_goodbye method on the value instance
}

#[test]
fn test_multiple_traits() {
    let person = Person {
        first_name: String::from("Hafidz"),
        last_name: String::from("Zhafir"),
        age: 25,
    };
    say_hello_goodbye_trait(&person); // Passing a reference to the person instance to the say_hello_goodbye_trait function

    // println!("{}", person.say_hello("Budi");)
    CanSayHello::say_hello(&person); // Calling the say_hello method from the CanSayHello trait on the person instance
    CanSayGoodBye::say_goodbye(&person); // Calling the say_goodbye method from the CanSayGoodBye trait on the person instance
    Person::say_hello(&person, "Budi"); // Calling the say_hello method on the person instance with a name argument
}

#[test]
fn test_trait_function() {
    let person = Person {
        first_name: String::from("Zhafir Rasyid"),
        last_name: String::from("Muhammad Hafidz"),
        age: 25,
    };
    say_hello_trait(&person); // Passing a reference to the person instance to the say_hello_trait function
}

struct SimplePerson {
    name: String,
}

impl CanSayGoodBye for SimplePerson {
    fn goodbye(&self) -> String {
        format!("Goodbye from SimplePerson: {}!", self.name)
    }
    fn say_goodbye(&self) -> String {
        format!("Goodbye from SimplePerson: {}!", self.name)
    }
    fn say_goodbye_to(&self, name: &str) -> String {
        format!("Goodbye, {}! From SimplePerson: {}.", name, self.name)
    }
}

impl CanSayHello for SimplePerson {
    fn say_hello(&self) -> String {
        format!("Hello from SimplePerson: {}!", self.name)
    }
    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello, {}! From SimplePerson: {}.", name, self.name)
    }
}

fn create_person(name: String) -> impl CanSayGoodBye {
    SimplePerson {
        name, // Creating an instance of SimplePerson with the provided name
    }
}

#[test]
fn test_return_trait() {
    let person = create_person(String::from("Zhafir Hafidz")); // Calling the create_person function to create a SimplePerson instance
    println!("{}", person.say_goodbye()); // Calling the say_goodbye method on the person instance
    println!("{}", person.goodbye()); // Calling the goodbye method on the person instance
    println!("{}", person.say_goodbye_to("Budi")); // Calling the say_goodbye_to method on the person instance
}

trait CanSay: CanSayHello + CanSayGoodBye {
    // This trait combines both CanSayHello and CanSayGoodBye traits
    fn say(&self) -> String;
}

impl CanSay for SimplePerson {
    // Implementing the CanSay trait for the Person struct
    fn say(&self) -> String {
        // This method combines the greetings from both traits
        format!("{}\n{} from CanSay", self.say_hello(), self.say_goodbye())
    }
}

fn say_hello_goodbye(person: &impl CanSay) {
    // This function takes a reference to any type that implements the CanSay trait
    println!("{}", person.say()); // Calling the say method on the person instance
}
#[test]
fn test_can_say() {
    let person = SimplePerson {
        name: String::from("Zhafir Hafidz"),
    };
    say_hello_goodbye(&person); // Passing a reference to the person instance to the say_hello_goodbye function
    println!("{}", person.say()); // Calling the say method on the person instance
}

struct Point<T> {
    x: T, // Generic type parameter T for the x coordinate
    y: T, // Generic type parameter T for the y coordinate
}

impl<T> Point<T> {

    fn get_x(&self) -> &T {
        // This method returns a reference to the x coordinate of the Point
        &self.x
    }

    fn get_y(&self) -> &T {
        // This method returns a reference to the y coordinate of the Point
        &self.y
    }

}

#[test]
fn test_generic_struct() {
    let point_i32: Point<i32> = Point { x: 10, y: 20 }; // Creating a Point with i32 type
    println!("Point i32: ({}, {})", point_i32.x, point_i32.y);

    let point_f64: Point<f64> = Point { x: 1.5, y: 2.5 }; // Creating a Point with f64 type
    println!("Point f64: ({}, {})", point_f64.x, point_f64.y);
}

enum Value<T> {
    NONE,
    VALUE(T),
}

#[test]
fn test_generic_enum() {
    let value_i32: Value<i32> = Value::<i32>::VALUE(42); // Creating a Value enum with i32 type
    let value_f64: Value<f64> = Value::<f64>::VALUE(3.14); // Creating a Value enum with f64 type

    match value_i32 {
        Value::NONE => println!("No value"),
        Value::VALUE(v) => println!("Value i32: {}", v),
    }

    match value_f64 {
        Value::NONE => println!("No value"),
        Value::VALUE(v) => println!("Value f64: {}", v),
    }
}

struct Hi<T: CanSayGoodBye> {
    value: T,
}

#[test]
fn test_generic_bound() {
    let hi = Hi::<SimplePerson> {
        value: SimplePerson {
            name: String::from("Zhafir"),
        },
    };
    println!("{}", hi.value.name)
}

fn min<T: PartialOrd>(value1: T, value2: T) -> T {
    // This function takes two values of type T and returns the minimum value
    if value1 < value2 { value1 } else { value2 }
}

#[test]
fn test_generic_function() {
    let result = min::<i32>(10, 20); // Calling the min function with i32 type
    println!("Minimum value (i32): {}", result);

    let result = min(20.1, 10.5); // Calling the min function with f64 type, Rust infers the type
    println!("Minimum value (f64): {}", result);
}

#[test]
fn test_generic_method() {
    let point = Point { x: 10, y: 20 }; // Creating a Point with i32 type
    println!("Point x: {}, y: {}", point.get_x(), point.get_y()); // Calling the get_x and get_y methods

    let point_f64 = Point { x: 1.5, y: 2.5 }; // Creating a Point with f64 type
    println!("Point f64 x: {}, y: {}", point_f64.get_x(), point_f64.get_y()); // Calling the get_x and get_y methods

    println!("Point x value: {}", point.get_value()); // Calling the get_value method to get the x coordinate
}

trait GetValue<T> {
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for Point<T> {
    fn get_value(&self) -> &T {
        &self.x // This method returns a reference to the x coordinate of the Point
    }
}

use core::ops::Add; // Importing the Add trait to implement the `+` operator for the Apple struct

struct Apple {
    quantity: i32,
}

impl Add for Apple {
    type Output = Apple; // Defining the output type of the addition operation

    fn add(self, rhs: Self) -> Self::Output {
        // Implementing the addition operation for Apple
        Apple {
            quantity: self.quantity + rhs.quantity, // Adding the quantities of the two Apple instances
        }
    }
}

#[test]
fn test_operator_add() {
    let apple1 = Apple {quantity: 5};
    let apple2 = Apple {quantity: 10};
    let result = apple1 + apple2; // Using the `+` operator to add two Apple instances
    println!("Total quantity of apples: {}", result.quantity); // This will print the total quantity of apples after addition
}