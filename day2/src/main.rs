use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct LineParser<T> {
  iter: io::Lines<io::BufReader<File>>,
  parser: fn(&String) -> Option<T>,
}

impl<T> LineParser<T> {
  fn new(iter: io::Lines<io::BufReader<File>>, parser: fn(&String) -> Option<T>) -> Self {
    return LineParser {
      iter: iter,
      parser: parser,
    };
  }
}

impl<T> Iterator for LineParser<T> {
  type Item = T;

  fn next(&mut self) -> Option<T> {
    match self.iter.next() {
      None => None,
      Some(result) => match result.ok() {
        None => None,
        Some(line) => (self.parser)(&line),
      },
    }
  }
}

fn parse_lines<S: AsRef<Path>, T>(
  s: S,
  p: fn(&String) -> Option<T>,
) -> Result<LineParser<T>, io::Error> {
  let file = File::open(s)?;
  let reader = io::BufReader::new(file).lines();
  let parser = LineParser::new(reader, p);
  Ok(parser)
}

fn parse(s: &String) -> Option<(String, i64)> {
  let mut parts = s.split_whitespace();
  let command = parts.next()?;
  let arg = parts.next()?.parse().ok()?;
  Some((command.to_string(), arg))
}

fn main() {
  let mut aim = 0;
  let mut horizontal = 0;
  let mut depth = 0;
  if let Ok(lines) = parse_lines("input.txt", parse) {
    for command in lines {
      match command.0.as_ref() {
        "forward" => {
          horizontal += command.1;
          depth += aim * command.1;
        }
        "up" => aim -= command.1,
        "down" => aim += command.1,
        _ => {}
      }
    }
  }
  println!("{}", horizontal * depth);
}
