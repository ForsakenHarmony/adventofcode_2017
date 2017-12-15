#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day15::{INPUT_A, INPUT_B};

fn main() {
  let res = day(INPUT_A, INPUT_B);
  println!("{:?}", res);
}

pub fn day(input_a: usize, input_b: usize) -> usize {
  let (mut gen1, mut gen2) = (input_a, input_b);
  let (step_a, step_b) = (16807, 48271);
  let mut pairs = 0;
  for i in 0..40_000_000 {
    gen1 = gen1 * step_a % 2147483647;
    gen2 = gen2 * step_b % 2147483647;

    if gen1 as u16 == gen2 as u16 {
      pairs += 1;
    }
  }
  pairs
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day15(b: &mut Bencher) {
    b.iter(|| {
      let input_a = test::black_box(INPUT_A);
      let input_b = test::black_box(INPUT_B);
      day(input_a, input_b)
    });
  }
}
