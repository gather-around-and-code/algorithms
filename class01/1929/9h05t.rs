use std::io;

fn main() {
  let limit = 100;
  // let mut line = String::new();
  // io::stdin().read_line(&mut line).expect("");

  let mut is_prime = vec![true; limit + 1];
  is_prime[0] = false;
  if limit >= 1 {
    is_prime[1] = false 
  }

  for num in 2..limit+1 {
    if is_prime[num] {
      let mut multiple = num * num;
      println!("{}", multiple);
      while multiple <= limit {
        is_prime[multiple] = false;
        multiple = num;
      }
    }
  }

  let v:Vec<usize> = is_prime.iter().enumerate()
    .filter_map(|(pr, &is_pr)| if is_pr { Some(pr) } else {None})
    .collect();

  for i in 5..10 {
    if is_prime[i] {
      println!("{}", v[i]);
    }
  }

  // let inputs: Vec<usize> = line.split_whitespace()
  // .map(|f| f.parse().expect(""))
  // .collect();

  // let mut m:usize = inputs[0];
  // let mut n:usize = inputs[1];

  // for index in m..n {
  //   if is_prime[index] {
  //     println!("{}", index);
  //   }
  // }
}