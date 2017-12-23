#![feature(test)]
extern crate test;
extern crate rayon;

extern crate adventofcode_2017;

use rayon::prelude::*;

use adventofcode_2017::day20::INPUT;

use std::ops::AddAssign;

fn main() {
  let res = day(
    "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>");
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
    }
  }).collect::<Vec<_>>();

  particles.par_iter_mut().map(|p| {
    for _ in 0..1_000 {
      p.step();
    }
    p.dist()
  }).enumerate().min_by_key(|p| (p.1)).unwrap().0
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day20(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
