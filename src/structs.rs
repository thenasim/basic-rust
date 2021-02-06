// Structs - Used to create custom data types

// Traditional Struct
// struct Color {
//   red: u8,
//   green: u8,
//   blue: u8,
// }

// Tuple Struct
// struct Color(u8, u8, u8);

struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    }
  }

  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }

  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() {
  // let mut c = Color {
  //   red: 23,
  //   green: 40,
  //   blue: 200,
  // };

  // c.blue = 100;

  // println!("Color: red={}, green={}, blue={}", c.red, c.green, c.blue);

  // ? Tuple struct
  // let mut clr = Color(255, 255, 255);

  // clr.0 = 0;

  // println!("Color: red={}, green={}, blue={}", clr.0, clr.1, clr.2);

  // ? Person struct
  let mut p1 = Person::new("Nasim", "Uddin");

  // println!("Full name: {} {}", p1.first_name, p1.last_name);
  println!("Full name: {}", p1.full_name());
  p1.set_last_name("Ahmed");
  println!("Full name: {}", p1.full_name());
  println!("Tuple: {:?}", p1.to_tuple());
}
