#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day4::INPUT;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &'static str) -> u32 {
  input
    .split('\n')
    .map(|s| {
      return s
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    })
    .filter_map(check_duplicates)
    .sum()
}

fn check_duplicates(pass: Vec<&str>) -> Option<u32> {
  for (i, string) in pass.iter().enumerate() {
    for compare in pass[0..i].iter() {
      if string == compare {
        return None;
      }
    }
  }
  Some(1)
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  
  #[bench]
  fn day4(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
