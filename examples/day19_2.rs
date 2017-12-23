#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day19::INPUT;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

struct Runner {
  grid: Vec<Vec<char>>,
  steps: usize,
  dir: (isize, isize),
  pos: (isize, isize),
}

impl Runner {
  fn new(grid: Vec<Vec<char>>) -> Runner {
    Runner {
      grid,
      steps: 1,
      dir: (0, 1),
      pos: (0, 0),
    }
  }

  fn get(&self, pos: &(isize, isize)) -> char {
    if pos.1 < 0 || pos.1 >= self.grid.len() as isize {
      return ' ';
    }
    let row = &self.grid[pos.1 as usize];
    if pos.0 < 0 || pos.0 >= row.len() as isize {
      return ' ';
    }
    let chr = row[pos.0 as usize];
    chr
  }

  fn step(&mut self) -> char {
    self.pos = (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1);
    self.get(&self.pos)
  }

  fn change_dir(&mut self) {
    let dirs = match self.dir {
      (1, 0) => {
        [
          (1, 0),
          (0, 1),
          (0, -1),
        ]
      }
      (-1, 0) => {
        [
          (-1, 0),
          (0, 1),
          (0, -1),
        ]
      }
      (0, 1) => {
        [
          (1, 0),
          (-1, 0),
          (0, 1),
        ]
      }
      (0, -1) => {
        [
          (1, 0),
          (-1, 0),
          (0, -1),
        ]
      }
      _ => {
        [(0, 0), (0, 0), (0, 0)]
      }
    };

    self.dir = *dirs.iter().find(|&dir| {
      let c = self.get(&(self.pos.0 + dir.0, self.pos.1 + dir.1));
      c != ' '
    }).unwrap_or(&(0, 0));
  }

  fn run(&mut self) -> usize {
    self.pos.0 = self.grid[0].iter().position(|&c| c == '|').unwrap() as isize;

    loop {
      if self.dir == (0,0) {
        break;
      }
      match self.step() {
        '+' => {
          self.change_dir();
        }
        '|' | '-' => {}
        ' ' => {
          break;
        }
        _ => {}
      }
      self.steps += 1;
    }

    self.steps
  }
}

pub fn day(input: &str) -> usize {
  let grid = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
  let mut runner = Runner::new(grid);
  runner.run()
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day19(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
