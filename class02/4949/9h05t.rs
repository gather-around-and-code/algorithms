use std::{io};

fn main() {
  loop {
    let mut buf = String::new();
    let mut char_stack:Vec<char> = Vec::new();

    io::stdin().read_line(&mut buf).expect("");
    let s = buf;

    if s.chars().nth(0) == Some('.') {
      return;
    }

    for i in s.chars() {
      if i == '(' || i == '[' {
        char_stack.push(i);
        continue;
      }

      if i == ')' {
        if (!char_stack.is_empty()) && (char_stack[char_stack.len() - 1] == '(') {
          char_stack.pop();
        }
        else { 
          println!("no"); 
          break;
        }
      }

      if i == ']' {
        if (!char_stack.is_empty()) && (char_stack[char_stack.len() - 1] == '[') {
          char_stack.pop();
        }
        else {
            println!("no");
            break;
        }
      }


      if char_stack.is_empty() && i == '.' {
        println!("yes");
      }
      else if !char_stack.is_empty() && i == '.' {
        println!("no");
      }
    }
  }
}