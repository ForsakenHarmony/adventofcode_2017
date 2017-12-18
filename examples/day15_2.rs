#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day15::{INPUT_A, INPUT_B};

fn main() {
  let res = day(INPUT_A, INPUT_B);
  println!("{:?}", res);
}

struct Generator {
  value: usize,
  step: usize,
  mod_check: usize,
}

impl Generator {
  fn is_valid(&self) -> bool {
    self.value % self.mod_check == 0
  }
}

impl Iterator for Generator {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    self.value = self.value * self.step % 2147483647;
    Some(self.value)
  }
}

pub fn day(input_a: usize, input_b: usize) -> usize {
  let mut gen_a = Generator {
    value: input_a,
    step: 16807,
    mod_check: 4,
  };
  let mut gen_b = Generator {
    value: input_b,
    step: 48271,
    mod_check: 8,
  };
  let mut pairs = 0;
  for i in 0..5_000_000 {
    while !gen_a.is_valid() {
      gen_a.next();
    }
    while !gen_b.is_valid() {
      gen_b.next();
    }
    if gen_a.value as u16 == gen_b.value as u16 {
      pairs += 1;
    }
    gen_a.next();
    gen_b.next();
  }
  pairs
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day15_2(b: &mut Bencher) {
    b.iter(|| {
      let input_a = test::black_box(INPUT_A);
      let input_b = test::black_box(INPUT_B);
      day(input_a, input_b)
    });
  }
}
