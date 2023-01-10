use std::{io};

fn main() {
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess);

    let s = guess
    .split_ascii_whitespace();

    let mut count = 0;
    for c in s {
        count = count + 1;
    }

    println!("{}", count);
}