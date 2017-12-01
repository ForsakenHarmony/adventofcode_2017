extern crate adventofcode_2017;
use adventofcode_2017::day1::INPUT;

fn main() {
  let arr = INPUT.as_bytes().iter().map(|char| {
    return (char - 48) as u64;
  }).collect::<Vec<_>>();
  let mut last = arr.last().unwrap();
  let mut sum = 0u64;
  for n in arr.iter() {
    if last == n {
      sum += n;
    }
    last = n;
  };
  println!("{:?}", sum);
}
