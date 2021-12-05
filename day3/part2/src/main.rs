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

fn balance_at_index(strings: &Vec<String>, index: usize) -> i64 {
  let mut balance = 0;
  for s in strings {
    match s.as_bytes()[index] as char {
      '1' => balance += 1,
      '0' => balance -=1,
      _ => {}
    }

  }
  balance
}

fn main() {
  if let Ok(reader) = read_lines("input.txt") {
    let mut strings: Vec<String> = reader.collect();
    for index in 0..12 {
      let balance = balance_at_index(&strings, index);
      strings = strings.iter().filter(|s| {
        let c = s.as_bytes()[index] as char;
        match c {
          '1' => balance < 0,
          '0' => balance >= 0,
          _ => false
        }
       }).cloned().collect();

       if strings.len() == 1 {
         break;
       }
    }
    assert_eq!(1, strings.len());
    println!("{}", u32::from_str_radix(&strings[0], 2).unwrap());
  }
}
