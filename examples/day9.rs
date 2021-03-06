#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day9::INPUT;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &str) -> usize {
  let mut chars = input.chars();

  let mut is_garbage = false;
  let mut counter = 0;
  let mut depth = 0;

  while let Some(c) = chars.next() {
    if is_garbage {
      match c {
        '!' => {
          chars.next();
        }
        '>' => {
          is_garbage = false;
        }
        _ => {}
      }
      continue;
    }
    match c {
      '{' => {
        depth += 1;
        counter += depth;
      }
      '}' => {
        depth -= 1;
      }
      '<' => {
        is_garbage = true;
      }
      _ => {}
    }
  }

  counter
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day9(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
