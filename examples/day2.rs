extern crate adventofcode_2017;

use adventofcode_2017::day2::INPUT;

fn main() {
  let arr: u32 = INPUT
    .split('\n')
    .map(|s| {
      return s
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    })
    .map(|row| calc_checksum(row))
    .sum();
  println!("{:?}", arr)
}

fn calc_checksum(row: Vec<u32>) -> u32 {
  let res = row.iter().fold((9999, 0), |mut acc, &val| {
    if val < acc.0 {
      acc.0 = val
    }
    if val > acc.1 {
      acc.1 = val
    }
    acc
  });
  
  res.1 - res.0
}
