mod first; // Importing the first module
mod second; // Importing the second module
mod model; // Importing the model module
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

fn mutable_reference(first_name: &mut String, last_name: &mut String) -> String { // &mut String allows modification of the original strings
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

    let person2: Person = Person {..person }; // Using struct update syntax to create a new instance of Person with the same values as `person`
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
    fn new(long: f64, lat: f64) -> GeoPoint { // associated function to create a new instance of GeoPoint
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
    let _nothing2: Nothing = Nothing{};
}

impl Person  {
    fn say_hello(&self, name: &str) { // self is a reference to the instance of Person // &self means we are borrowing the instance, not taking ownership
        // This method takes a reference to self and a string slice as an argument
        println!("Hello, {}! My name is {} {} and I am {} years old.", name, self.first_name, self.last_name, self.age);
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

    match level { // match expression to handle different variants of the enum
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
    let _payment2: Payment = Payment::BankTransfer(String::from("Bank A"), String::from("1234567890")); // Creating an instance of the Payment enum with BankTransfer variant
    let _payment3: Payment = Payment::EWallet(String::from("BCA"), String::from("1234567890")); // Creating an instance of the Payment enum with EWallet variant
}

impl Payment {
    fn pay(&self, amount: u32) { // enum method to process payment
        match self {
            Payment::CreditCard(number ) => {
                println!("Processing credit card payment of {} with card number: {}", amount, number);
            }
            Payment::BankTransfer(bank, account) => {
                println!("Processing bank transfer payment of {} from bank: {} with account number: {}", amount, bank, account);
            }
            Payment::EWallet(provider, account) => {
                println!("Processing e-wallet payment of {} from provider: {} with account number: {}", amount, provider, account);
            }
            
        }
    }
}

#[test]
fn test_payment_method() {
    let _payment1: Payment = Payment::CreditCard(String::from("1234-5678-9012-3456")); // Creating an instance of the Payment enum with CreditCard variant
    let _payment2: Payment = Payment::BankTransfer(String::from("Bank A"), String::from("1234567890")); // Creating an instance of the Payment enum with BankTransfer variant
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
        Person { first_name, last_name, .. } => {
            println!("Person: {} {}", first_name, last_name);
        }
    }
}

#[test]
fn test_ignoring() {
    let point = GeoPoint::new(0.0, 1.0);

    match point {
        GeoPoint(long, _) => { // Ignoring the latitude value
            println!("Point is on the x-axis at longitude: {}", long);
        }
        GeoPoint(_, lat) => { // Ignoring the longitude value
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
