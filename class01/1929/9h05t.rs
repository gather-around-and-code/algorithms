use std::io;

fn main() {
  let mut line = String::new();
  io::stdin().read_line(&mut line).expect("");

  let inputs: Vec<i32> = line.split_whitespace()
  .map(|f| f.parse().expect(""))
  .collect();

  let mut m = inputs[0];
  let mut n = inputs[1];

  let mut arr:[i32;10000000];
  let stop:i32 = arr.len() as i32;

  // how can initilize array in Rust?
  for i in 0..stop {
    arr[i] = i;
  }
}