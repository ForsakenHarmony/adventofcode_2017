#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day17::INPUT;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: usize) -> usize {
  let mut current_index = 0;
  let mut after0 = 1;
  for len in 1..50_000_001 {
    current_index = (current_index + input) % len + 1;
    if current_index == 1 {
      after0 = len;
    }
  }
  after0
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day17_2(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
