use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct LineReader {
  iter: io::Lines<io::BufReader<File>>,
}

impl Iterator for LineReader {
  type Item = String;

  fn next(&mut self) -> Option<String> {
    let next = self.iter.next();
    match next {
      None => None,
      Some(result) => result.ok(),
    }
  }
}

fn read_lines<S: AsRef<Path>>(s: S) -> Result<LineReader, io::Error> {
  let file = File::open(s)?;
  let iter = io::BufReader::new(file).lines();
  Ok(LineReader { iter: iter })
}

struct Balance {
  at_index: [i32; 12],
}

impl Balance {
  fn new() -> Balance {
    Balance { at_index: [0; 12] }
  }

  fn to_number(self: &Self) -> u64 {
    let mut result: u64 = 0;
    for (i, item) in self.at_index.iter().rev().enumerate() {
      if *item > 0 {
        let exponent: u32 = i.try_into().unwrap();
        result += u64::pow(2, exponent);
      }
    }
    result
  }
}

fn main() {
  let mut balance = Balance::new();
  if let Ok(reader) = read_lines("input.txt") {
    for line in reader {
      assert_eq!(line.len(), balance.at_index.len());
      let mut i = 0;
      for char in line.chars() {
        match char {
          '1' => balance.at_index[i] += 1,
          '0' => balance.at_index[i] -= 1,
          _ => {}
        }
        i += 1;
      }
    }
  }
  let gamma = balance.to_number();
  let epsilon = !(balance.to_number()) & u64::pow(2, 12) - 1;
  println!("{}", gamma * epsilon);
}
