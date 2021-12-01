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

struct SlidingWindow {
  iter: NumericLines,
  ready: bool,
  window: [i64; 3],
}

impl SlidingWindow {
  fn new(iter: NumericLines) -> SlidingWindow {
    SlidingWindow {
      iter: iter,
      ready: false,
      window: [0; 3],
    }
  }
}

impl Iterator for SlidingWindow {
  type Item = [i64; 3];

  fn next(&mut self) -> Option<Self::Item> {
    if !self.ready {
      self.window = [0, self.iter.next()?, self.iter.next()?];
      self.ready = true;
    }
    self.window[0] = self.window[1];
    self.window[1] = self.window[2];
    self.window[2] = self.iter.next()?;
    Some(self.window)
  }
}

fn read_numbers<S: AsRef<Path>>(s: S) -> Result<SlidingWindow, io::Error> {
  let file = File::open(s)?;
  let reader = io::BufReader::new(file).lines();
  let numbers = NumericLines { iter: reader };
  Ok(SlidingWindow::new(numbers))
}

fn main() {
  let mut increasing = 0;
  let mut previous: Option<i64> = None;
  if let Ok(lines) = read_numbers("input.txt") {
    for numbers in lines {
      let sum = numbers.iter().sum();
      match previous {
        Some(value) => {
          if sum > value {
            increasing += 1;
          }
        }
        None => {}
      }
      previous = Some(sum);
    }
  }
  println!("{}", increasing);
}
