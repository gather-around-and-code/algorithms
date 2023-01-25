use std::{io, usize};
fn main() {
  let max = 1000001;
  let mut buf = String::new();
  io::stdin().read_line(&mut buf)
  .expect("");

  let s: Vec<usize> = buf.split_ascii_whitespace().into_iter().map(|x| x.parse::<usize>().unwrap_or(0)).collect();
  let mut primes: Vec<bool> = Vec::new();
  for _ in 0..max {
    primes.push(true);
  }
  
  let m = (max as f64).sqrt() as usize;
  for i in 2..m+1 {
    if primes[i] {
      for j in (i+i..max).step_by(i) {
        primes[j] = false;
      }
    }
  }

  for i in s[0]..=s[1] {
    if primes[i] {
      println!("{}", i.to_string());
    }
  }
}