/* Generic Data Types in Rust

In Rust, we can define generic data types for:
- Functions
- Structs and Methods
- Enums

Generic data types are useful when we want to write code that can work with multiple types.
*/

// Generic for Structs and methods
struct Point<T, U> {
  x: T,
  y: U,
} 

//get x and y values
impl <T, U> Point<T, U> {
  fn x(&self) -> &T {
      &self.x
  }

  fn y(&self) -> &U {
      &self.y
  }
  
}

//distance from origin
impl Point<f32, f32> {
  fn distance_from_origin(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

//generics type parameters aren't always the same type
// we used in the struct definition
//mixer method
impl<T, U> Point<T, U> {
  fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
      Point {
          x: self.x,
          y: other.y,
      }
  }
}

fn main() {
  let integer = Point { x: 5, y: 10 };
  let float = Point { x: 1.0, y: 4.2 };
  let char = Point { x: 'a', y: 'b' };

  println!("integer.x = {}, integer.y = {}", integer.x, integer.y);
  println!("float.x = {}, float.y = {}", float.x, float.y);
  println!("char.x = {}, char.y = {}", char.x, char.y);

  //can we provide different types for a Point<T> struct?
  let mixed = Point { x: 5, y: 4.2 };
  println!("mixed.x = {}, mixed.y = {}", mixed.x, mixed.y);

  //get x value for integer
  println!("integer.x = {}", integer.x());

  //get y value for float
  println!("float.y = {}", float.y());

  //distance from origin for integer
  let p4 = Point { x: 3.0, y: 4.0 };
  println!("Distance from origin for p4: {}", p4.distance_from_origin());

  //distance from origin for float
  let p5 = Point { x: 3.0, y: 4.0 };
  println!("Distance from origin for p5: {}", p5.distance_from_origin());

  //distance from origin for char
  // let p6 = Point { x: 'a', y: 'b' };
  // println!("Distance from origin for p6: {}", p6.distance_from_origin());

  //mixup method
  //define pMix1
  let pMix1 = Point { x: 5, y: 10 };
  //define pMix2
  let pMix2 = Point { x: 1.0, y: 4.2 };

  //mixup pMix1 and pMix2
  let pMix3 = pMix1.mixup(pMix2);
  println!("pMix3.x = {}, pMix3.y = {}", pMix3.x, pMix3.y);

}
