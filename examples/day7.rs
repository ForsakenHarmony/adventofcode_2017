#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day7::INPUT;

use std::collections::HashMap;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &'static str) -> &str {
  let mut current  = "";
  let mut map = HashMap::new();

  for line in input.lines() {
    let mut split = line.splitn(2, " ");
    let name = split.next().unwrap();
    let rem = split.next().unwrap().split("-> ").skip(1).next();
    if let Some(children) = rem {
      for child in children.split(", ") {
        map.insert(child, name);
      }
      current = name;
    }
  }

  while let Some(next) = map.get(current) {
    current = next;
  }
  
  current
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  
  #[bench]
  fn day7(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
