#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day23::INPUT;

use std::collections::HashMap;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

#[derive(Debug)]
enum Instruction {
  SetR(char, char),
  Set(char, isize),
  SubR(char, char),
  Sub(char, isize),
  MulR(char, char),
  Mul(char, isize),
  JgzN(isize),
  Jgz(char, isize),
  JgzR(char, char),
}

pub fn day(input: &str) -> isize {
  let mut registers: HashMap<char, isize> = HashMap::new();

  let instructions = input.lines().filter_map(|l| {
    let mut split = l.split_whitespace();
    let op = split.next().unwrap();
    let x = split.next().unwrap();
    match op.as_ref() {
      "set" => {
        let y = split.next().unwrap();
        if let Ok(n) = y.parse::<isize>() {
          Some(Instruction::Set(x.chars().nth(0).unwrap(), n))
        } else {
          Some(Instruction::SetR(x.chars().nth(0).unwrap(), y.chars().nth(0).unwrap()))
        }
      }
      "sub" => {
        let y = split.next().unwrap();
        if let Ok(n) = y.parse::<isize>() {
          Some(Instruction::Sub(x.chars().nth(0).unwrap(), n))
        } else {
          Some(Instruction::SubR(x.chars().nth(0).unwrap(), y.chars().nth(0).unwrap()))
        }
      }
      "mul" => {
        let y = split.next().unwrap();
        if let Ok(n) = y.parse::<isize>() {
          Some(Instruction::Mul(x.chars().nth(0).unwrap(), n))
        } else {
          Some(Instruction::MulR(x.chars().nth(0).unwrap(), y.chars().nth(0).unwrap()))
        }
      }
      "jnz" => {
        let y = split.next().unwrap();
        if let Ok(n) = y.parse::<isize>() {
          if let Ok(m) = x.parse::<isize>() {
            if m != 0 {
              Some(Instruction::JgzN(n))
            } else {
              None
            }
          } else {
            Some(Instruction::Jgz(x.chars().nth(0).unwrap(), n))
          }
        } else {
          Some(Instruction::JgzR(x.chars().nth(0).unwrap(), y.chars().nth(0).unwrap()))
        }
      }
      _ => { None }
    }
  }).collect::<Vec<Instruction>>();

  let mut i: isize = 0;
  let mut mul_count = 0;

  loop {
    if i < 0 || i >= instructions.len() as isize {
      break;
    }
    println!("{:?}", instructions[i as usize]);
    match instructions[i as usize] {
      Instruction::SetR(r, rf) => {
        let v = registers.entry(rf).or_insert(0).clone();
        let entry = registers.entry(r).or_insert(0);
        *entry = v;
      }
      Instruction::Set(r, v) => {
        let entry = registers.entry(r).or_insert(0);
        *entry = v;
      }
      Instruction::SubR(r, rf) => {
        let v = registers.entry(rf).or_insert(0).clone();
        let entry = registers.entry(r).or_insert(0);
        *entry -= v;
      }
      Instruction::Sub(r, v) => {
        let entry = registers.entry(r).or_insert(0);
        *entry -= v;
      }
      Instruction::MulR(r, rf) => {
        let v = registers.entry(rf).or_insert(0).clone();
        let entry = registers.entry(r).or_insert(0);
        *entry *= v;
        mul_count += 1;
      }
      Instruction::Mul(r, v) => {
        let entry = registers.entry(r).or_insert(0);
        *entry *= v;
        mul_count += 1;
      }
      Instruction::JgzR(r, rf) => {
        let v = *registers.entry(rf).or_insert(0);
        let entry = *registers.entry(r).or_insert(0);
        if entry != 0 {
          i += v - 1;
        }
      }
      Instruction::JgzN(v) => {
        i += v - 1;
      }
      Instruction::Jgz(r, v) => {
        let entry = *registers.entry(r).or_insert(0);
        if entry != 0 {
          i += v - 1;
        }
      }
    }
    i += 1;
  }

  mul_count
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day23(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
