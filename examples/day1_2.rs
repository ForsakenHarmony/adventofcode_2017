#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day1::INPUT;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &str) -> u64 {
  let arr = input.as_bytes().iter().map(|char| {
    return (char - 48) as u64;
  }).collect::<Vec<_>>();
  let len = arr.len();
  let mut sum = 0u64;
  for (i, n) in arr.iter().enumerate() {
    if n == &arr[(i + len / 2) % len] {
      sum += n;
    }
  };
  sum
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  
  #[bench]
  fn day1_2(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
