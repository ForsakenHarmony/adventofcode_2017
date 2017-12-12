#![feature(test)]
extern crate test;

extern crate adventofcode_2017;

use adventofcode_2017::day12::INPUT;

use std::collections::{HashMap, HashSet};

fn main() {
  let res = day(INPUT);
  println!("{:?}", res);
}

struct Node {
  connections: HashSet<usize>,
  visited: bool,
}

impl Node {
  fn new(connections: HashSet<usize>) -> Node {
    Node {
      connections,
      visited: false,
    }
  }

  fn visit(&mut self) {
    self.visited = true;
  }
}

pub fn day(input: &str) -> usize {
  let mut graph: HashMap<usize, Node> = HashMap::new();

  for line in input.lines() {
    let mut split = line.split(" <-> ");
    let id = split.next().unwrap().parse::<usize>().unwrap();
    let connections = split.next().unwrap().split(", ").map(|s| s.parse::<usize>().unwrap()).collect::<HashSet<_>>();

    graph.insert(id, Node::new(connections));
  }

  find_conns(&mut graph, 0)
}

fn find_conns(graph: &mut HashMap<usize, Node>, from: usize) -> usize {
  let mut check_next = Vec::new();
  if let Some(node) = graph.get_mut(&from) {
    if node.visited {
      return 0;
    } else {
      node.visit();
    }
    for &conn in node.connections.iter() {
      check_next.push(conn);
    }
  } else { panic!("Node at id: {} not found", from); }

  let sum: usize = check_next.iter().map(|&c| find_conns(graph, c)).sum();
  sum + 1
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn day12(b: &mut Bencher) {
    b.iter(|| {
      let input = test::black_box(INPUT);
      day(input)
    });
  }
}
