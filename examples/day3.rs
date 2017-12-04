extern crate adventofcode_2017;

use adventofcode_2017::day3::INPUT;

fn main() {
  let (x, y) = coord_from_num(INPUT);
  println!("{}, {} -> {}", x, y, x.abs() + y.abs());
}

fn coord_from_num(n: i32) -> (i32, i32) {
  let (ring, max) = find_ring_and_max(n);
  let width = ring * 2;
  let dist = (max - n) % width + 1;
  let offset = width / 2 - 1;
  (ring, width - offset - dist)
}

fn find_ring_and_max(n: i32) -> (i32, i32) {
  let mut ring = 0;
  loop {
    let max = max_for_ring(ring);
    if max >= n {
      return (ring, max);
    }
    ring += 1;
  }
}

fn max_for_ring(ring: i32) -> i32 {
  (1i32 + ring * 2i32).pow(2)
}
