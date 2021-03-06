#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day6::INPUT;

use std::collections::HashMap;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &str) -> u64 {
  let mut vec = input
    .split_whitespace()
    .filter(|s| !s.is_empty())
    .map(|s| s.parse::<u32>().unwrap())
    .collect::<Vec<_>>();
  
  let mut map = HashMap::new();
  
  let mut counter: u64 = 0;
  
  map.insert(vec.clone(), counter);

  let index = loop {
    vec = balance(vec);
    counter += 1;
    if let Some(v) = map.insert(vec.clone(), counter) {
      break v
    }
  };
  
  counter - index
}

fn balance(mut vec: Vec<u32>) -> Vec<u32> {
  let mut biggest = 0;
  let mut biggest_i = 0;
  for (i, &n) in vec.iter().enumerate() {
    if n > biggest {
      biggest = n;
      biggest_i = i;
    }
  }
  vec[biggest_i] = 0;
  
  let len = vec.len();
  
  for i in 0..biggest as usize {
    vec[(biggest_i + i + 1) % len] += 1;
  }
  
  vec
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  
  #[bench]
  fn day6_2(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
