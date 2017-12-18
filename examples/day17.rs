#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day17::INPUT;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: usize) -> usize {
  let mut buffer = vec![0];
  let mut current_index = 0;
  for v in 1..2018 {
    let len = buffer.len();
    current_index = (current_index + input + 1) % len;
    buffer.insert(current_index, v);
  }
  println!("{:?}", buffer);
  let i = buffer.iter().position(|&v| v == 2017).unwrap();
  buffer[(i + 1) % buffer.len()]
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day17(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
