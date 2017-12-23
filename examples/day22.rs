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
struct Carrier {
  pos: Point,
  dir: Point,
  map: HashMap<Point, bool>,
  infected: usize,
}

impl Carrier {
  fn new(map: HashMap<Point, bool>) -> Carrier {
    Carrier {
      infected: 0,
      dir: Point(0, -1),
      pos: Point(0, 0),
      map,
    }
  }

  fn burst(&mut self) {
    let infected = self.map.entry(self.pos).or_insert(false);

    println!("pos: {:?}, dir: {:?}, infected: {}, curr_inf: {}", self.pos, self.dir, self.infected, infected);

    if *infected {
      self.dir = match self.dir {
        Point(0, 1) => Point(-1, 0),
        Point(0, -1) => Point(1, 0),
        Point(1, 0) => Point(0, 1),
        Point(-1, 0) => Point(0, -1),
        _ => Point(0, 0),
      };
      *infected = false;
    } else {
      self.infected += 1;
      self.dir = match self.dir {
        Point(0, 1) => Point(1, 0),
        Point(0, -1) => Point(-1, 0),
        Point(1, 0) => Point(0, -1),
        Point(-1, 0) => Point(0, 1),
        _ => Point(0, 0),
      };
      *infected = true;
    }

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
      println!("({}, {}): {:?}", x, y, c);
      map.insert(Point(x, y), if c == '#' { true } else { false });
    }
  }
  let mut carrier = Carrier::new(map);
  for _ in 0..10_000 {
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
