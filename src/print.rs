pub fn run() {
  println!("Hello form print.rs");

  // ? Basic formatting
  println!("Number is {}", 1);

  // ? Positional argument
  println!("{} is form {}", "Nasim", "Bangladesh");

  // ? Named argument
  println!("{name}'s age is {age}", name = "Mehedi", age = 18);

  // ? Placeholder tratis
  println!("Binary: {:b}, Octal: {:o}, Hex: {:x}", 10, 10, 10);

  // ? Placeholder for debug tratis
  println!("{:?}", (22, true, "hello"));

  // ? Basic math
  println!("10 + 20 = {}", 10 + 20);
}
