extern crate adventofcode_2017;

use adventofcode_2017::day4::INPUT;

fn main() {
  let arr: u32 = INPUT
    .split('\n')
    .map(|s| {
      return s
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    })
    .filter_map(check_duplicates)
    .sum();
  println!("{:?}", arr)
}

fn check_duplicates(pass: Vec<&str>) -> Option<u32> {
  for (i, string) in pass.iter().enumerate() {
    for compare in pass[0..i].iter() {
      if string == compare {
        return None
      }
    }
  }
  Some(1)
}
