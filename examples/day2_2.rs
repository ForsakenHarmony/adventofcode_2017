#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day2::INPUT;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &str) -> f32 {
  input
    .split('\n')
    .map(|s| {
      return s
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<f32>().unwrap())
        .collect::<Vec<f32>>();
    })
    .map(|row| evenly_divisible(row))
    .sum()
}

fn evenly_divisible(row: Vec<f32>) -> f32 {
  for (i, n) in row.iter().enumerate() {
    for (j, m) in row.iter().enumerate() {
      if i == j { continue };
      if n < m { continue };
      
      if (n / m) % 1.0 == 0.0 {
        return n / m;
      }
    }
  }
  0.0
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  
  #[bench]
  fn day2_2(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
