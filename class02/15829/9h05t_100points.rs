use std::io;

fn main() {
  let r = 31;
  let M = 1234567891;
  let mut buf = String::new();

  io::stdin().read_line(&mut buf)
    .expect("");
  let n = buf.trim_end().parse::<usize>()
    .expect("");
  buf.clear();

  io::stdin().read_line(&mut buf)
    .expect("");
  let a = buf.trim_end().as_bytes()
    .iter()
    .map(|&b| (b - b'a' as u8 + 1) as usize)
    .collect::<Vec<_>>();

  let mut res = 0;
  for (i, &c) in a.iter().enumerate() {
    let mut pow = 1;
    // From rolling hashing formular, each character cacculated.
    for _ in 0..i {
      // From Sigma modular lawer, each value to calculate a modular
      pow = pow * r % M;
    }
    res = (res + c * pow);
  }

  println!("{}", res % M);
}