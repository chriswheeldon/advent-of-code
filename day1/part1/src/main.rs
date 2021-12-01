use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct NumericLines {
  iter: io::Lines<io::BufReader<File>>,
}

impl Iterator for NumericLines {
  type Item = i64;

  fn next(&mut self) -> Option<Self::Item> {
    match self.iter.next() {
      None => None,
      Some(result) => match result.ok() {
        None => None,
        Some(line) => line.parse().ok(),
      },
    }
  }
}

fn read_numbers<S: AsRef<Path>>(s: S) -> Result<NumericLines, io::Error> {
  let file = File::open(s)?;
  let reader = io::BufReader::new(file).lines();
  Ok(NumericLines { iter: reader })
}

fn main() {
  let mut increasing = 0;
  let mut previous: Option<i64> = None;
  if let Ok(lines) = read_numbers("input.txt") {
    for number in lines {
      match previous {
        Some(value) => {
          if number > value {
            increasing += 1;
          }
        }
        None => {}
      }
      previous = Some(number);
    }
  }
  println!("{}", increasing);
}
