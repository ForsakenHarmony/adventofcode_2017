#![feature(test)]
extern crate test;

fn main() {
  let res = day();
  println!("{:?}", res);
}

pub fn day() -> isize {
  let mut b = 106_500;
  let c = b + 17_000;
  let mut h = 0;

  while b <= c {
    for d in 2..b {
      if b % d == 0 {
        h += 1;
        break;
      }
    }

    b += 17;
  }

  h
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day23(b: &mut Bencher) {
    b.iter(|| {
      day()
    });
  }
}
