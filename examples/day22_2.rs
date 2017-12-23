#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day22::INPUT;

use std::ops::AddAssign;
use std::collections::HashMap;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point(isize, isize);

impl AddAssign for Point {
  fn add_assign(&mut self, rhs: Point) {
    self.0 += rhs.0;
    self.1 += rhs.1;
  }
}

#[derive(Debug)]
enum State {
  Clean,
  Weakened,
  Infected,
  Flagged,
}

#[derive(Debug)]
struct Carrier {
  pos: Point,
  dir: Point,
  map: HashMap<Point, State>,
  infected: usize,
}

impl Carrier {
  fn new(map: HashMap<Point, State>) -> Carrier {
    Carrier {
      infected: 0,
      dir: Point(0, -1),
      pos: Point(0, 0),
      map,
    }
  }

  fn burst(&mut self) {
    let cur_state = self.map.entry(self.pos).or_insert(State::Clean);

//    println!("pos: {:?}, dir: {:?}, infected: {}, curr_inf: {:?}", self.pos, self.dir, self.infected, cur_state);

    *cur_state = match *cur_state {
      State::Clean => {
        self.dir = match self.dir {
          Point(0, 1) => Point(1, 0),
          Point(0, -1) => Point(-1, 0),
          Point(1, 0) => Point(0, -1),
          Point(-1, 0) => Point(0, 1),
          _ => Point(0, 0),
        };
        State::Weakened
      },
      State::Weakened => {
        self.infected += 1;
        State::Infected
      },
      State::Infected => {
        self.dir = match self.dir {
          Point(0, 1) => Point(-1, 0),
          Point(0, -1) => Point(1, 0),
          Point(1, 0) => Point(0, 1),
          Point(-1, 0) => Point(0, -1),
          _ => Point(0, 0),
        };
        State::Flagged
      },
      State::Flagged => {
        self.dir = match self.dir {
          Point(0, 1) => Point(0, -1),
          Point(0, -1) => Point(0, 1),
          Point(1, 0) => Point(-1, 0),
          Point(-1, 0) => Point(1, 0),
          _ => Point(0, 0),
        };
        State::Clean
      },
    };

    self.pos += self.dir;
  }
}

pub fn day(input: &str) -> usize {
  let mut map = HashMap::new();
  for (y, l) in input.lines().enumerate() {
    let offset = ((l.len() as isize) - 1) / 2;
    let y = (y as isize) - offset;
    for (x, c) in l.chars().enumerate() {
      let x = (x as isize) - offset;
      map.insert(Point(x, y), if c == '#' { State::Infected } else { State::Clean });
    }
  }
  let mut carrier = Carrier::new(map);
  for _ in 0..10_000_000 {
    carrier.burst();
  }
  carrier.infected
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day2(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
