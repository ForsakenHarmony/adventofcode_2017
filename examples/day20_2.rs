#![feature(test)]
extern crate test;
extern crate rayon;

extern crate adventofcode_2017;

use rayon::prelude::*;

use adventofcode_2017::day20::INPUT;

use std::ops::AddAssign;
use std::collections::HashMap;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point(isize, isize, isize);

impl AddAssign for Point {
  fn add_assign(&mut self, rhs: Point) {
    self.0 += rhs.0;
    self.1 += rhs.1;
    self.2 += rhs.2;
  }
}

#[derive(Debug)]
struct Particle {
  pos: Point,
  vel: Point,
  acc: Point,
  exists: bool,
}

impl Particle {
  fn dist(&self) -> isize {
    self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs()
  }

  fn step(&mut self) {
    self.vel += self.acc;
    self.pos += self.vel;
  }
}

pub fn day(input: &str) -> usize {
  let mut particles = input.lines().map(|l| {
    let mut points = l.split(", ").map(|s| {
      let mut split = s[3..s.len() - 1].split(",").map(|n| n.parse::<isize>().unwrap());
      Point(split.next().unwrap(), split.next().unwrap(), split.next().unwrap())
    });
    Particle {
      pos: points.next().unwrap(),
      vel: points.next().unwrap(),
      acc: points.next().unwrap(),
      exists: true,
    }
  }).collect::<Vec<_>>();

  for _ in 0..1_000 {
    let mut map = HashMap::new();
    for i in 0..particles.len() {
      match map.insert(particles[i].pos, i) {
        Some(j) => {
          particles[i].exists = false;
          particles[j].exists = false;
        }
        None => {}
      }
      particles[i].step();
    }
  }

  particles.iter().filter(|p| p.exists).count()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day20_2(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
