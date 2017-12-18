#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day18::INPUT;

use std::collections::HashMap;

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

enum Instruction {
  Snd(char),
  SetR(char, char),
  Set(char, isize),
  AddR(char, char),
  Add(char, isize),
  MulR(char, char),
  Mul(char, isize),
  ModR(char, char),
  Mod(char, isize),
  Rcv(char),
  JgzR(char, char),
  JgzN(isize),
  Jgz(char, isize),
}

struct Program {
  registers: HashMap<char, isize>,
  i: isize,
  buffer: Vec<isize>,
}

impl Program {
  fn new(id: isize) -> Program {
    let mut registers: HashMap<char, isize> = HashMap::new();
    registers.insert('p', id);
    Program {
      registers,
      i: 0,
      buffer: Vec::new()
    }
  }

  fn send(&mut self, v: isize) {
    self.buffer.push(v);
  }

  fn run_until_rcv(&mut self, instructions: &Vec<Instruction>) -> Vec<isize> {
    let mut to_send = Vec::new();

    loop {
      if self.i < 0 || self.i > instructions.len() as isize {
        panic!("wew");
      }
      match instructions[self.i as usize] {
        Instruction::Snd(r) => {
          to_send.push(self.registers.entry(r).or_insert(0).clone());
        }
        Instruction::SetR(r, rf) => {
          let v = self.registers.entry(rf).or_insert(0).clone();
          let entry = self.registers.entry(r).or_insert(0);
          *entry = v;
        }
        Instruction::Set(r, v) => {
          let entry = self.registers.entry(r).or_insert(0);
          *entry = v;
        }
        Instruction::AddR(r, rf) => {
          let v = self.registers.entry(rf).or_insert(0).clone();
          let entry = self.registers.entry(r).or_insert(0);
          *entry += v;
        }
        Instruction::Add(r, v) => {
          let entry = self.registers.entry(r).or_insert(0);
          *entry += v;
        }
        Instruction::MulR(r, rf) => {
          let v = self.registers.entry(rf).or_insert(0).clone();
          let entry = self.registers.entry(r).or_insert(0);
          *entry *= v;
        }
        Instruction::Mul(r, v) => {
          let entry = self.registers.entry(r).or_insert(0);
          *entry *= v;
        }
        Instruction::ModR(r, rf) => {
          let v = self.registers.entry(rf).or_insert(0).clone();
          let entry = self.registers.entry(r).or_insert(0);
          *entry %= v;
        }
        Instruction::Mod(r, v) => {
          let entry = self.registers.entry(r).or_insert(0);
          *entry %= v;
        }
        Instruction::Rcv(r) => {
          let entry = self.registers.entry(r).or_insert(0);
          if self.buffer.len() > 0 {
            *entry = self.buffer.remove(0);
          } else {
            break;
          }
        }
        Instruction::JgzR(r, rf) => {
          let v = *self.registers.entry(rf).or_insert(0);
          let entry = *self.registers.entry(r).or_insert(0);
          if entry > 0 {
            self.i += v - 1;
          }
        }
        Instruction::JgzN(v) => {
          self.i += v - 1;
        }
        Instruction::Jgz(r, v) => {
          let entry = *self.registers.entry(r).or_insert(0);
          if entry > 0 {
            self.i += v - 1;
          }
        }
      }
      self.i += 1;
    }

    to_send
  }
}

pub fn day(input: &str) -> usize {
  let instructions = input.lines().filter_map(|l| {
    let mut split = l.split_whitespace();
    let op = split.next().unwrap();
    let x = split.next().unwrap();
    match op.as_ref() {
      "snd" => {
        Some(Instruction::Snd(x.chars().nth(0).unwrap()))
      }
      "set" => {
        let y = split.next().unwrap();
        if let Ok(n) = y.parse::<isize>() {
          Some(Instruction::Set(x.chars().nth(0).unwrap(), n))
        } else {
          Some(Instruction::SetR(x.chars().nth(0).unwrap(), y.chars().nth(0).unwrap()))
        }
      }
      "add" => {
        let y = split.next().unwrap();
        if let Ok(n) = y.parse::<isize>() {
          Some(Instruction::Add(x.chars().nth(0).unwrap(), n))
        } else {
          Some(Instruction::AddR(x.chars().nth(0).unwrap(), y.chars().nth(0).unwrap()))
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
      "mod" => {
        let y = split.next().unwrap();
        if let Ok(n) = y.parse::<isize>() {
          Some(Instruction::Mod(x.chars().nth(0).unwrap(), n))
        } else {
          Some(Instruction::ModR(x.chars().nth(0).unwrap(), y.chars().nth(0).unwrap()))
        }
      }
      "rcv" => {
        Some(Instruction::Rcv(x.chars().nth(0).unwrap()))
      }
      "jgz" => {
        let y = split.next().unwrap();
        if let Ok(n) = y.parse::<isize>() {
          if let Ok(m) = x.parse::<isize>() {
            if m > 0 {
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

  let mut p0 = Program::new(0);
  let mut p1 = Program::new(1);

  let mut count = 0;

  loop {
    let res = p0.run_until_rcv(&instructions);
    for v in res { p1.send(v); }
    let res = p1.run_until_rcv(&instructions);
    for v in res { p0.send(v); count += 1; }
    if p0.buffer.len() == 0 && p1.buffer.len() == 0 {
      break;
    }
  }

  count
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day18(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
