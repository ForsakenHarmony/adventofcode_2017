extern crate adventofcode_2017;

use adventofcode_2017::day2::INPUT;

fn main() {
  let arr: f32 = INPUT
    .split('\n')
    .map(|s| {
      return s
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<f32>().unwrap())
        .collect::<Vec<f32>>();
    })
    .map(|row| evenly_divisible(row))
//    .collect::<Vec<_>>();
    .sum();
  println!("{:?}", arr)
}

fn evenly_divisible(row: Vec<f32>) -> f32 {
  for (i, n) in row.iter().enumerate() {
    for (j, m) in row.iter().enumerate() {
      if i == j { continue };
      if n < m { continue };
      if (n / m) % 1f32 == 0f32 {
        return n / m
      }
    }
  }
  0f32
}
