/* Generic Data Types in Rust

In Rust, we can define generic data types for:
- Functions
- Structs
- Methods
- Enums


Generic data types are useful when we want to write code that can work with multiple types.
*/

//Generics for Enums

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let some_number = Option::Some(5);
    let no_number: Option<i32> = Option::None;

    println!("{:?}", some_number);
    println!("{:?}", no_number);

    let success: Result<i32, &str> = Result::Ok(5);
    let failure: Result<i32, &str> = Result::Err("Error");

    match success {
        Result::Ok(value) => println!("Success: {}", value),
        Result::Err(error) => println!("Error: {}", error),
    }

    match failure {
        Result::Ok(value) => println!("Success: {}", value),
        Result::Err(error) => println!("Error: {}", error),
    }
}
