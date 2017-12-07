#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day5::INPUT;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &'static str) -> u32 {
  let mut instructions = input
    .split_whitespace()
    .filter(|s| !s.is_empty())
    .map(|s| s.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
  
  let mut counter = 0;
  let mut index = 0;
  
  while index < instructions.len() {
    let mov = instructions[index];
    instructions[index] = mov + if mov >= 3 { -1 } else { 1 };
    index = (index as i32 + mov) as usize;
    
    counter += 1;
  }
  
  counter
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  
  #[bench]
  fn day5_2(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
