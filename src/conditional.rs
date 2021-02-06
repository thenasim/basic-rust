pub fn run() {
  let age: u8 = 19;
  let is_deshi: bool = true;

  if age > 17 && is_deshi {
    println!("You are allowed for passport");
  } else if age < 18 && is_deshi {
    println!("You are a kid!");
  } else {
    println!("Foriegner not allowed");
  }

  // ? Short hand if
  let is_of_age = if age >= 18 { true } else { false };

  println!("Is of age: {}", is_of_age);
}
