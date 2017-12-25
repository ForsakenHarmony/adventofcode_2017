#![feature(test)]
extern crate test;
extern crate regex;

extern crate adventofcode_2017;

use regex::Regex;
use std::collections::HashMap;

use adventofcode_2017::day25::INPUT;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &str) -> usize {
  let begin_re = Regex::new(r"Begin\sin\sstate\s(\w)").unwrap();
  let steps_re = Regex::new(r"after\s(\d+)\ssteps").unwrap();

  let mut curr_state = begin_re.captures(input).unwrap().get(1).unwrap().as_str().chars().nth(0).unwrap();
  let steps = steps_re.captures(input).unwrap()[1].parse::<usize>().unwrap();

  let state_map = {
    let state_re = Regex::new(r"(?xsm)
      In\sstate\s(\w)
      .+?is\s0
      .+?value\s(1|0)
      .+?to\sthe\s(right|left)
      .+?with\sstate\s(\w)
      .+?is\s1
      .+?value\s(1|0)
      .+?to\sthe\s(right|left)
      .+?with\sstate\s(\w)
    ").unwrap();

    let mut state_map = HashMap::new();

    for m in state_re.captures_iter(input) {
      let vec = m.iter().skip(1).map(|s| s.unwrap().as_str().chars().nth(0).unwrap()).collect::<Vec<_>>();

      state_map.insert(vec[0], ((vec[1].to_digit(10).unwrap() as usize, vec[2], vec[3]), (vec[4].to_digit(10).unwrap() as usize, vec[5], vec[6])));
    }

    state_map
  };

  let mut tape: HashMap<isize, usize> = HashMap::new();
  let mut index = 0;

  for _ in 0..steps {
    let state = state_map.get(&curr_state).unwrap();

    let entry = tape.entry(index).or_insert(0);

    let thing = if *entry == 0 { state.0 } else { state.1 };

    *entry = thing.0;
    index += match thing.1 {
      'r' => 1,
      'l' => -1,
      _ => 0
    };
    curr_state = thing.2;
  }

  tape.values().sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day25(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
