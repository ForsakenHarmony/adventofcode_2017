extern crate adventofcode_2017;
use adventofcode_2017::day1::INPUT;

fn main() {
  let arr = INPUT.as_bytes().iter().map(|char| {
    return (char - 48) as u64;
  }).collect::<Vec<_>>();
  let len = arr.len();
  let mut sum = 0u64;
  for (i, n) in arr.iter().enumerate() {
    if n == &arr[(i + len / 2) % len] {
      sum += n;
    }
  };
  println!("{:?}", sum);
}
