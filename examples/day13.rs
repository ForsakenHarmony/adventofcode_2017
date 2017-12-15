#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day13::INPUT;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &str) -> usize {
  input.lines().filter_map(|l| {
    let mut split = l.split(": ").map(|s| s.parse::<usize>().unwrap());
    let depth = split.next().unwrap();
    let range = split.next().unwrap();

    let s_pos = depth % (range * 2 - 2);

    if s_pos == 0 {
      Some(depth * range)
    } else {
      None
    }
  }).sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day13(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
