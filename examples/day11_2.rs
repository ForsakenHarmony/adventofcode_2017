#![feature(test)]
extern crate test;
//extern crate rayon;

extern crate adventofcode_2017;

use adventofcode_2017::day11::INPUT;

//use rayon::prelude::*;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

pub fn day(input: &str) -> isize {
  let mut split = input.split(",");

  let (mut n, mut s, mut ne, mut se, mut nw, mut sw)
  = (0isize, 0isize, 0isize, 0isize, 0isize, 0isize);

  let mut max_dist = 0;

  while let Some(dir) = split.next() {
    match dir.as_ref() {
      "n" => if s > 0 { s -= 1 } else { n += 1 }
      "s" => if n > 0 { n -= 1 } else { s += 1 }
      "ne" => if sw > 0 { sw -= 1 } else { ne += 1 }
      "se" => if nw > 0 { nw -= 1 } else { se += 1 }
      "nw" => if se > 0 { se -= 1 } else { nw += 1 }
      "sw" => if ne > 0 { ne -= 1 } else { sw += 1 }
      _ => {}
    }

    for _ in 0..2 {
      n += rdiff(&mut nw, &mut ne);
      ne += rdiff(&mut n, &mut se);
      se += rdiff(&mut ne, &mut s);
      s += rdiff(&mut se, &mut sw);
      sw += rdiff(&mut s, &mut nw);
      nw += rdiff(&mut sw, &mut n);
    }

    let dist = n + s + ne + se + nw + sw;

    max_dist = max_dist.max(dist);
  }

  max_dist
}

//pub fn day_rayon(input: &str) -> isize {
//  let split = input.split(",");
//
//  let (mut n, mut s, mut ne, mut se, mut nw, mut sw)
//  = (0isize, 0isize, 0isize, 0isize, 0isize, 0isize);
//
//  split.into_par_iter()
//    .map(|dir| {
//      match dir.as_ref() {
//        "n" => if s > 0 { s -= 1 } else { n += 1 }
//        "s" => if n > 0 { n -= 1 } else { s += 1 }
//        "ne" => if sw > 0 { sw -= 1 } else { ne += 1 }
//        "se" => if nw > 0 { nw -= 1 } else { se += 1 }
//        "nw" => if se > 0 { se -= 1 } else { nw += 1 }
//        "sw" => if ne > 0 { ne -= 1 } else { sw += 1 }
//        _ => {}
//      }
//
//      (n, s, ne, se, nw, sw)
//    })
//    .map(|(mut n, mut s, mut ne, mut se, mut nw, mut sw)| {
//      for _ in 0..2 {
//        n += rdiff(&mut nw, &mut ne);
//        ne += rdiff(&mut n, &mut se);
//        se += rdiff(&mut ne, &mut s);
//        s += rdiff(&mut se, &mut sw);
//        sw += rdiff(&mut s, &mut nw);
//        nw += rdiff(&mut sw, &mut n);
//      }
//
//      n + s + ne + se + nw + sw
//    })
//    .max().unwrap()
//}

fn rdiff(x: &mut isize, y: &mut isize) -> isize {
  let sub = if *x > 0 && *y >= *x {
    *x
  } else if *y > 0 && *x >= *y {
    *y
  } else {
    0
  };

  *x -= sub;
  *y -= sub;

  sub
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day11_2(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }

//  #[bench]
//  fn day11_rayon(b: &mut Bencher) {
//    b.iter(|| {
//      let input = test::black_box(INPUT);
//      day_rayon(input)
//    });
//  }
}
