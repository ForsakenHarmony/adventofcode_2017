#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day10::INPUT;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &str) -> usize {
  let mut lengths = input.split(",").map(|s| s.parse::<usize>().unwrap());
  let mut current_pos = 0;
  let mut skip = 0;
  let mut list = (0..256).collect::<Vec<usize>>();
  let list_len = list.len();

  while let Some(len) = lengths.next() {
    let reversed = (current_pos..current_pos + len).map(|pos| list[pos % list_len]).rev().collect::<Vec<usize>>();
    for (i, &n) in reversed.iter().enumerate() {
      list[(current_pos + i) % list_len] = n;
    }
    current_pos = (current_pos + len + skip) % list_len;
    skip += 1;
  }

  list[0] * list[1]
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day10(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
