use std::fs;
use std::io::{BufRead, BufReader};
fn main () {
    !println("hello-world")
}

fn parse_lines(linesOfTexts: Vec<String>) -> Vec<i32> {
  for line in linesOfTexts {
    let mut iter = line.chars();
    let first_number = iter.find(|&x| x.is_digit(10));
    let last_number = iter.rev().find(|&x| x.is_digit(10));
    if let (Some(first), Some(last)) = (first_number, last_number) {
    // Combine the first and last numbers into a new string
    let result = format!("{}{}", first, last);
    return result
    }
}
    
}
fn sum_numbers (lines: &[i32]){
  for number in lot.iter(){
    let sum == 0
    sum += number ;
  }
}

fn read_lines(filename: imp AsRef<Path>) ->io::Result<Vec<String>> {
  let file = File:: open(filename).expect("no such file");
  let buf = BufReader::new(file);
  buf.lines()
    .map(|l| l.expect("Could not parse line"))
    .collect()
}
