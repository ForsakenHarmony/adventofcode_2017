#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day9::INPUT;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &str) -> usize {
  let mut split = input.split("");

  let mut is_garbage = false;
  let mut counter = 0;
  let mut depth = 0;

  while let Some(s) = split.next() {
    let s: &str = s;
    if s.is_empty() { continue };
    if is_garbage {
      match s.as_ref() {
        "!" => {
          split.next();
        }
        ">" => {
          is_garbage = false;
        }
        _ => {}
      }
      continue;
    }
    match s.as_ref() {
      "{" => {
        depth += 1;
        counter += depth;
      }
      "}" => {
        depth -= 1;
      }
      "<" => {
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
