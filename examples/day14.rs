#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day14::INPUT;
use std::ops::{BitAnd, Div, BitXor};

pub fn knot_hash(input: &str) -> String {
  let lengths = input.chars().map(|c| c as usize).chain([17, 31, 73, 47, 23].iter().map(|&n| n)).collect::<Vec<_>>();
  let mut current_pos = 0;
  let mut skip = 0;
  let mut list = (0..256).collect::<Vec<usize>>();
  let list_len = list.len();

  for _ in 0..64 {
    let mut lengths = lengths.iter();
    while let Some(len) = lengths.next() {
      let reversed = (current_pos..current_pos + len).map(|pos| list[pos % list_len]).rev().collect::<Vec<usize>>();
      for (i, &n) in reversed.iter().enumerate() {
        list[(current_pos + i) % list_len] = n;
      }
      current_pos = (current_pos + len + skip) % list_len;
      skip += 1;
    };
  }

  (0..list_len / 16)
    .map(|i|
      list[i * 16..i * 16 + 16]
        .iter()
        .fold(0u8, |acc, &val| acc.bitxor(val as u8)))
    .map(|n| format!("{:01$x}", n, 2))
    .fold("".to_owned(), |acc, val| acc + &val)
}

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &str) -> usize {
  (0usize..128usize).map(|i| {
    let hash = knot_hash(format!("{}-{}", input, i).as_ref());
    hash
      .chars()
      .map(|c| c.to_digit(16).unwrap() as u8)
      .map(|b| b.bitand(1) + b.bitand(2).div(2) + b.bitand(4).div(4) + b.bitand(8).div(8))
      .sum::<u8>() as usize
  }).sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day14(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
