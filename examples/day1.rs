#![feature(test)]
extern crate test;

extern crate adventofcode_2017;
use adventofcode_2017::day1::INPUT;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &'static str) -> u64{
  let arr = input.as_bytes().iter().map(|char| {
    return (char - 48) as u64;
  }).collect::<Vec<_>>();
  let mut last = arr.last().unwrap();
  let mut sum = 0u64;
  for n in arr.iter() {
    if last == n {
      sum += n;
    }
    last = n;
  };
  sum
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  
  #[bench]
  fn day1(b: &mut Bencher) {
    b.iter(|| day(INPUT));
  }
}
