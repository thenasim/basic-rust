/**
  Primitive Types--
  Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
  Floats: f32, f64
  Boolean (bool)
  Characters (char)
  Tuples
  Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it

pub fn run() {
  // ? Default is i32
  let x = 1;

  // ? Default is f64
  let y = 2.5;

  // ? Explicit type
  let age: u8 = 65;

  // ? Find max size
  println!("i32 max size: {}", std::i32::MAX);

  // ? Boolean
  let is_active = false;

  // ? Boolean from expression
  let is_adult = age > 17;

  // ? Character
  let a = 'a';
  let happy = '\u{1F600}';

  println!("{:?}", (is_active, is_adult, x, y, age, a, happy));
}
