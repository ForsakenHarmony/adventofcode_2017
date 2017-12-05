#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day3::INPUT;
use std::collections::HashMap;

fn main() {
  let res = day(INPUT);
  println!("{}", res);
}

pub fn day(input: i32) -> i32 {
  let check = vec![
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1)
  ];
  
  let mut positions: HashMap<(i32, i32), i32> = HashMap::new();
  positions.insert((0, 0), 1);
  let mut last = (0, 0);
  let mut last_n = 1;
  let mut ring = 0;
  
  while last_n < input {
    let (x, y) = last;
    let max = (x as i32).abs().max((y as i32).abs());
    if max > ring {
      ring += 1;
    }
    
    last = if y == -ring {
      (x + 1, y)
    } else if x == -ring {
      (x, y - 1)
    } else if y == ring {
      (x - 1, y)
    } else {
      (x, y + 1)
    };
    
    last_n = check.iter().filter_map(|c| positions.get(&(c.0 + last.0, c.1 + last.1))).sum();
    positions.insert(last, last_n);
  };
  
  last_n
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  
  #[bench]
  fn day3_2(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
