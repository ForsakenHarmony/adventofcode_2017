#![feature(test)]
#![feature(vec_remove_item)]

extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day24::INPUT;

fn main() {
  let res = day(INPUT);
  println!("{}", res);
}

pub fn day(input: &str) -> usize {
  let conns = input.lines().map(|s| {
    let mut split = s.split("/").map(|n| n.parse::<usize>().unwrap());
    (split.next().unwrap(), split.next().unwrap())
  }).collect::<Vec<_>>();

  max_len_bridge(0, conns).1
}

fn max_len_bridge(from: usize, conns: Vec<(usize, usize)>) -> (usize, usize) {
  conns.iter().filter_map(|c| {
    if c.0 == from {
      let recur = max_len_bridge(c.1, {
        let mut new_vec = conns.clone();
        new_vec.remove_item(c);
        new_vec
      });

      Some((recur.0 + 1, recur.1 + c.0 + c.1))
    } else if c.1 == from {
      let recur = max_len_bridge(c.0, {
        let mut new_vec = conns.clone();
        new_vec.remove_item(c);
        new_vec
      });

      Some((recur.0 + 1, recur.1 + c.0 + c.1))
    } else {
      None
    }
  }).max_by_key(|i| i.0).unwrap_or((0, 0))
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day24_2(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
