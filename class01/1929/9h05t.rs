use std::{io, usize};
use std::fmt::Write;
fn main() {
  let max = 1000001;
  let mut buf = String::new();
  io::stdin().read_line(&mut buf)
  .expect("");

  let s: Vec<usize> = buf.split_ascii_whitespace().into_iter().map(|x| x.parse::<usize>().unwrap_or(0)).collect();
  let mut is_prime = vec![true; s[1] as usize + 1];
  is_prime[0] = false;
  is_prime[1] = false;

  let mut c = 2 as usize;
  let m = (s[1] as f64).sqrt() as usize + 1;

  while c <= m as usize {
      if is_prime[c] {
        let mut mul = 2;
        let mut i = c * mul;

        while i <= s[1] as usize {
          is_prime[i] = false;
          mul += 1;
          i = c * mul;
        }
      }
      c += 1
  }

  let mut res = String::new();
  for i in s[0]..=s[1] {
    if is_prime[i] {
      writeln!(res, "{}", i).unwrap();
    }
  }

  println!("{}", res);
}