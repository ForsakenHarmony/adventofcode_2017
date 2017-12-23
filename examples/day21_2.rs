#![feature(test)]
extern crate test;
extern crate rayon;

extern crate adventofcode_2017;

use rayon::prelude::*;

use adventofcode_2017::day21::INPUT;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

#[derive(Debug)]
struct Image {
  size: usize,
  data: Vec<char>,
}

impl Image {
  fn new() -> Image {
    Image {
      size: 3,
      data: vec!['.', '#', '.', '.', '.', '#', '#', '#', '#'],
    }
  }

  fn get(&self, x: usize, y: usize) -> char {
    self.data[y * self.size + x]
  }

  fn debug(data: &Vec<char>) -> String {
    let w = (data.len() as f64).sqrt() as usize;
    data.iter().enumerate().fold(String::new(), |mut s, (i, &c)| {
      if i % w == 0 && i > 0 {
        s.push('\n');
      }
      s.push(c);
      s
    })
  }

  fn split(&mut self) -> Vec<Vec<char>> {
    let s = self.size;
    if s % 2 == 0 {
      let sections_side = s / 2;
      let sections: usize = sections_side.pow(2);

      (0..sections).map(|i| {
        let x = (i % sections_side) * 2;
        let y = (i - (i % sections_side)) / sections_side * 2;
        vec![
          self.get(x, y),
          self.get(x + 1, y),
          self.get(x, y + 1),
          self.get(x + 1, y + 1)
        ]
      }).collect()
    } else if s % 3 == 0 {
      let sections_side = s / 3;
      let sections: usize = sections_side.pow(2);

      (0..sections).map(|i| {
        let x = (i % sections_side) * 3;
        let y = (i - (i % sections_side)) / sections_side * 3;
        vec![
          self.get(x, y),
          self.get(x + 1, y),
          self.get(x + 2, y),
          self.get(x, y + 1),
          self.get(x + 1, y + 1),
          self.get(x + 2, y + 1),
          self.get(x, y + 2),
          self.get(x + 1, y + 2),
          self.get(x + 2, y + 2),
        ]
      }).collect()
    } else {
      Vec::new()
    }
  }

  fn match_section(&self, section: Vec<char>, rules: &Vec<(Vec<char>, Vec<char>)>) -> Option<Vec<char>> {
    let mut s = section;
    if s.len() % 2 == 0 {
      for _ in 0..4 {
        s = vec![s[2], s[0], s[3], s[1]];
        if let Some(r) = rules.iter().find(|&r| r.0 == s) {
          return Some(r.1.clone());
        }
      }
    } else {
      for _ in 0..4 {
        if let Some(r) = rules.iter().find(|&r| r.0 == s || r.0 == flip(&s)) {
          return Some(r.1.clone());
        }
        // rotate
        s = vec![s[6], s[3], s[0], s[7], s[4], s[1], s[8], s[5], s[2]];
      }
    }
    panic!("couldn't find a match for: {:?}", s);
  }

  fn match_rules(&mut self, rules: &Vec<(Vec<char>, Vec<char>)>) {
    let sections = self.split();

    let sections = sections.par_iter().map(|&ref s|
      self.match_section(s.clone(), rules).unwrap()
    ).collect::<Vec<_>>();

    let s_w = (sections.len() as f64).sqrt() as usize;
    let s_l = if self.size % 2 == 0 { 3 } else { 4 };

    self.data.clear();

    for i in 0..s_w {
      let y = i * s_w;

      let p_sec = &sections[y..y + s_w];

      for y in 0..s_l {
        let y = y * s_l;
        for s in p_sec {
          s[y..y + s_l].iter().for_each(|&c| self.data.push(c));
        }
      }
    }

    self.size = self.size + s_w;
  }
}

fn flip (s: &Vec<char>) -> Vec<char> {
  vec![s[2], s[1], s[0], s[5], s[4], s[3], s[8], s[7], s[6]]
}

pub fn day(input: &str) -> usize {
  let rules = input.lines().map(|s| {
    let mut split = s.split(" => ").map(|s| s.chars().filter(|&c| c != '/').collect::<Vec<_>>());
    (split.next().unwrap(), split.next().unwrap())
  }).collect::<Vec<_>>();

  let mut image = Image::new();

  for _ in 0..18 {
    image.match_rules(&rules);
  }

  image.data.iter().filter(|&&c| c == '#').count()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day21(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
