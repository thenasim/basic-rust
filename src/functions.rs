pub fn run() {
  let age: i8 = 34;
  greetings("Welcome", age);

  // ? Bind functions to variables
  let get_sum = add(10, 30);
  println!("10 + 30 = {}", get_sum);

  // ? Closure - can use outside variables
  let c = 3;
  let sub_num = |a: i32, b: i32| a - b - c;
  println!("40 - 20 - {1} = {0}", sub_num(40, 20), c);
}

fn greetings(greet: &str, age: i8) {
  println!("{} Nasim, your age is {}", greet, age);
}

fn add(a: i32, b: i32) -> i32 {
  return a + b;
}
