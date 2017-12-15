#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day14::INPUT;
use std::ops::{BitAnd, BitXor};
use std::collections::HashMap;

pub fn knot_hash(input: &str) -> String {
  let lengths = input.chars().map(|c| c as usize).chain([17, 31, 73, 47, 23].iter().map(|&n| n)).collect::<Vec<_>>();
  let mut current_pos = 0;
  let mut skip = 0;
  let mut list = (0..256).collect::<Vec<usize>>();
  let list_len = list.len();

  for _ in 0..64 {
    let mut lengths = lengths.iter();
    while let Some(len) = lengths.next() {
      let reversed = (current_pos..current_pos + len).map(|pos| list[pos % list_len]).rev().collect::<Vec<usize>>();
      for (i, &n) in reversed.iter().enumerate() {
        list[(current_pos + i) % list_len] = n;
      }
      current_pos = (current_pos + len + skip) % list_len;
      skip += 1;
    };
  }

  (0..list_len / 16)
    .map(|i|
      list[i * 16..i * 16 + 16]
        .iter()
        .fold(0u8, |acc, &val| acc.bitxor(val as u8)))
    .map(|n| format!("{:01$x}", n, 2))
    .fold("".to_owned(), |acc, val| acc + &val)
}

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &str) -> usize {
  let mut map = HashMap::new();

  for y in 0isize..128isize {
    let hash = knot_hash(format!("{}-{}", input, y).as_ref());
    for (x, v) in hash.chars().map(|c| c.to_digit(16).unwrap() as u8).enumerate() {
      let x = x as isize * 4;
      if v.bitand(1) > 0 { map.insert((x + 3, y), false); };
      if v.bitand(2) > 0 { map.insert((x + 2, y), false); };
      if v.bitand(4) > 0 { map.insert((x + 1, y), false); };
      if v.bitand(8) > 0 { map.insert((x + 0, y), false); };
    }
  }

  let mut count = 0;
  loop {
    if map.len() == 0 { break; }
    let item = map.keys().next().unwrap().clone();
    for conn in find_connected(&mut map, item) {
      map.remove(&conn);
    }
    count += 1;
  }
  count
}

fn find_connected(map: &mut HashMap<(isize, isize), bool>, from: (isize, isize)) -> Vec<(isize, isize)> {
  *map.get_mut(&from).unwrap() = true;
  let check_conns = vec![
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1)
  ];
  check_conns
    .iter()
    .filter_map(|c| {
      let pos = (from.0 + c.0, from.1 + c.1);
      let visited = *map.get(&pos)?;
      if visited {
        None
      } else {
        Some(find_connected(map, pos))
      }
    })
    .fold(vec![from], |acc, val| {
      [acc, val].concat()
    })
}
//  let mut check_next = check_conns
//    .iter()
//    .map(|c| (from.0 + c.0, from.1 + c.1))
//    .collect::<Vec<(isize, isize)>>();
//  while check_next.len() > 0 {
//    let clone = check_next.clone();
//    check_next.clear();
//    for pos in clone {
//      if let Some(square) = map.get_mut(&pos) {
//        if !square.visited {
//          square.visit();
//          for pos in check_conns
//            .iter()
//            .map(|c| (pos.0 + c.0, pos.1 + c.1)) {
//            check_next.push(pos);
//          }
//          connections.push(pos);
//        }
//      }
//    }
//  }
//  connections

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day14_2(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
