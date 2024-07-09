//Performance of Code using Generics in Rust

#![allow(unused)]
fn main() {
    let integer = Some(5);
    let float = Some(5.0);
}

/* 

When Rust compiles this code, it performs monomorphization. 
During monomorphization, the Rust compiler reads the values that have been used in the code and identifies the types used. 
It then generates the code for the specific types used in the code. 
This means that the code that is generated is specific to the types used in the code. 
This process is what makes Rust code using generics fast at runtime. */


//at compile time
enum Option_i32 {
  Some(i32),
  None,
}

enum Option_f64 {
  Some(f64),
  None,
}

fn main() {
  let integer = Option_i32::Some(5);
  let float = Option_f64::Some(5.0);
}

// The Generic Type T is replaced with the specific type used in the code.

