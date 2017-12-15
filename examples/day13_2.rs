#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day13::INPUT;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &str) -> usize {
  let firewall = input.lines().map(|l| {
    let mut split = l.split(": ").map(|s| s.parse::<usize>().unwrap());
    (split.next().unwrap(), split.next().unwrap())
  }).collect::<Vec<(usize, usize)>>();

  let mut offset = 0;
  while is_caught(&firewall, offset) {
    offset += 1
  }
  offset
}

fn is_caught(firewall: &Vec<(usize, usize)>, offset: usize) -> bool {
  for &(depth, range) in firewall.iter() {
    if (offset + depth) % (range * 2 - 2) == 0 {
      return true;
    }
  }
  false
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day13_2(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
