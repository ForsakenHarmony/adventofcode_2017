#![feature(test)]
#![feature(slice_rotate)]
#![feature(slice_patterns)]
#![feature(match_default_bindings)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day16::INPUT;
use std::char;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

#[derive(Debug)]
enum Dance {
  Spin(usize),
  Exchange(usize, usize),
  Partner(char, char)
}

pub fn day(input: &str) -> String {
  let mut state = (0..16).map(|n| char::from_u32(n + 97).unwrap()).collect::<Vec<char>>();

  let commands = input.split(",").map(|s| s.split_at(1)).filter_map(|(op, rest)| match op.as_ref() {
    "s" => Some(Dance::Spin(rest.parse::<usize>().unwrap())),
    "x" => {
      let mut split = rest.split("/").map(|s| s.parse::<usize>().unwrap());
      Some(Dance::Exchange(split.next().unwrap(), split.next().unwrap()))
    }
    "p" => {
      let split = rest.chars().collect::<Vec<char>>();
      Some(Dance::Partner(split[0], split[2]))
    }
    _ => None
  }).collect::<Vec<_>>();

  for instr in commands {
    match instr {
      Dance::Spin(off) => spin(&mut state, off),
      Dance::Exchange(first_i, second_i) => exchange(&mut state, first_i, second_i),
      Dance::Partner(first, second) => partner(&mut state, first, second),
    }
  }

  state.into_iter().collect::<String>()
}

fn spin(state: &mut Vec<char>, offset: usize) {
  let len = state.len();
  state.rotate(len - offset);
}

fn exchange(state: &mut Vec<char>, first_i: usize, second_i: usize) {
  let temp = state[first_i];
  state[first_i] = state[second_i];
  state[second_i] = temp;
}

fn partner(state: &mut Vec<char>, first: char, second: char) {
  let first_i = state.iter().position(|&v| v == first).unwrap();
  let second_i = state.iter().position(|&v| v == second).unwrap();
  exchange(state, first_i, second_i);
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day16(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
