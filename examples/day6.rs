#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day6::INPUT;

use std::collections::HashSet;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &'static str) -> u64 {
  let mut vec = input
    .split_whitespace()
    .filter(|s| !s.is_empty())
    .map(|s| s.parse::<u32>().unwrap())
    .collect::<Vec<_>>();
  
  let mut set = HashSet::new();
  
  set.insert(vec.clone());
  
  let mut counter: u64 = 0;
  
  println!("c: {} now: {:?}", counter, vec);
  loop {
    vec = balance(vec);
    counter += 1;
    println!("c: {} now: {:?}", counter, vec);
    if !set.insert(vec.clone()) {
      break
    }
  }
  
  counter
}

fn balance(mut slice: Vec<u32>) -> Vec<u32> {
  let mut biggest = 0;
  let mut biggest_i = 0;
  for (i, &n) in slice.iter().enumerate() {
    if n > biggest {
      biggest = n;
      biggest_i = i;
    }
  }
  slice[biggest_i] = 0;
  
  let len = slice.len();
  
  for i in 0..biggest as usize {
    slice[(biggest_i + i + 1) % len] += 1;
  }
  
  slice
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  
  #[bench]
  fn day6(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
