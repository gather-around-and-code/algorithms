use std::{io};

fn main() {
  loop {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("");
    let s = buf.trim();
    
    let mut char_stack:Vec<char> = Vec::new();

    for i in s.chars() {
      if i == '.' {
        return;
      }

      if i == '(' || i == '[' {
        char_stack.push(i);
      }

      if i == ')' {
        let c = char_stack.pop().unwrap();
        if c == '(' {
          continue;
        }
      }
      if i == ']' {
        let c = char_stack.pop().unwrap();
        if c == '[' {
          continue;
        }
      }
    }
  }
}
