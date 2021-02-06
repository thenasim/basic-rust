// Vectors - Resizable arrays

pub fn run() {
  let mut numbers: Vec<i16> = vec![1, 2, 3, 4];

  // Re-assign value
  numbers[2] = 30;

  // Add number to vector
  numbers.push(5);
  numbers.push(6);

  // Pop the last vlaue
  numbers.pop();

  println!("{:?}", numbers);

  // Get single val
  println!("Single Value: {}", numbers[0]);

  // Get vector length
  println!("Vector Length: {}", numbers.len());

  // Vectors are heap allocated
  println!("Vectors occupies {} bytes", std::mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i16] = &numbers[1..3];
  println!("Slice: {:?}", slice);

  // Loop through vectors
  for x in numbers.iter() {
    println!("{}", x);
  }

  // Loop & iterates
  for x in numbers.iter_mut() {
    *x *= 2;
  }

  println!("Numbers: {:?}", numbers);
}
