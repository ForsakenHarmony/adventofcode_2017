#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day8::INPUT;

use std::collections::HashMap;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &str) -> isize {
  let mut map: HashMap<&str, isize> = HashMap::new();
  let mut highest = 0;

  for line in input.lines() {
    let mut split = line.split_whitespace();
    let reg = split.next().unwrap();
    let op = split.next().unwrap();
    let n = split.next().unwrap().parse::<isize>().unwrap();
    split.next();
    let c_reg = split.next().unwrap();
    let cond = split.next().unwrap();
    let c_n = split.next().unwrap().parse::<isize>().unwrap();

    let c_reg_v = *map.get(c_reg).unwrap_or(&0);

    if !match cond.as_ref() {
      ">" => c_reg_v > c_n,
      "<" => c_reg_v < c_n,
      ">=" => c_reg_v >= c_n,
      "<=" => c_reg_v <= c_n,
      "==" => c_reg_v == c_n,
      "!=" => c_reg_v != c_n,
      _ => panic!("invalid cond")
    } {
      continue;
    }

    let mut reg_v = map.entry(reg).or_insert(0);

    match op.as_ref() {
      "inc" => *reg_v += n,
      "dec" => *reg_v -= n,
      _ => panic!("invalid op")
    }

    if *reg_v > highest {
      highest = *reg_v;
    }
  }

  highest
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day8_2(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
