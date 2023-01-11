use std::io;

fn main() {
  let mut s = String::new();

  io::stdin()
    .read_line(&mut s)
    .expect("");

  let n: i32 = s
  .trim()
  .parse()
  .expect("");

  for i in 0..n {
    for j in 0..i+1 {
      print!("*");
    }
    print!("\n");
  }
}