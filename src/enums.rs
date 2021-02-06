// ? Enums are types which have a few definite values

enum Movement {
  Up,
  Down,
  Left,
  Right,
}

fn move_robot(m: Movement) {
  match m {
    // match is kind of like switch
    Movement::Up => println!("Moving up"),
    Movement::Down => println!("Moving Down"),
    Movement::Left => println!("Moving Left"),
    Movement::Right => println!("Moving Right"),
  }
}

pub fn run() {
  let robot1 = Movement::Up;
  let robot2 = Movement::Down;
  let robot3 = Movement::Left;
  let robot4 = Movement::Right;

  move_robot(robot1);
  move_robot(robot2);
  move_robot(robot3);
  move_robot(robot4);
}
