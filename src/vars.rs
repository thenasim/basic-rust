// ? Variables hold primitive data or references to data
// ? Variables are immutable by default
// ? Rust is a block-scoped language

pub fn run() {
  let name = "Brad";
  let mut age = 37;

  println!("My name is {} and I am {}", name, age);
  age = 80;
  println!("My name is {} and I am {}", name, age);

  // ? Define constant
  const ID: i32 = 34;

  println!("ID: {}", ID);

  // ? Assign mutliple variable
  let (my_country, my_name) = ("BD", "MEHEDI HASAN");
  println!("I am {}, I live in {}", my_name, my_country);
}
