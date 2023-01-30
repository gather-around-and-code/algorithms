use std::{io};

fn main() {
  let r = 31;
  let M = 1234567891;
  let mut N = String::new();
  let mut buf = String::new();
  
  let n = match io::stdin().read_line(&mut N) {
    Ok(_) => N.trim().parse::<i32>().unwrap(),
    Err(_) => return,
  };

  let mut a = Vec::new();
  match io::stdin().read_line(&mut buf) {
    Ok(_) => {
      for i in buf.trim().as_bytes() {
        let t = (i - 96) as i32;
        a.push(t);
      }
    }
    Err(_) => return,
  };

  let mut res:i32 = 0;
  for i in 0..n {
    let c = a[i as usize] * i32::pow(r, i as u32);
    res += c;
    res %= M;
  }

  println!("{}", res.to_string());

}