use crate::third::say_hello as third_say_hello;

pub fn say_hello() {
    // This function prints a greeting message
    println!("Hello from the first module!");

    // Call the say_hello function from the third module
    third_say_hello();
}

mod second {
    mod third {
        pub fn say_hello() {
            crate::first::say_hello(); 

            super::super::say_hello(); // Call the say_hello function from the first module
            
        }
    }
}