mod ss;

use std::io::{self, Stdin};
use std::cmp;

fn find_joltage(content: &str) -> u32 {
  let bytes = content.as_bytes();
  let len = bytes.len();
  let mut max_rhs : u32 = 0;
  let mut ret : u32 = 0;
  for i in (0..len).rev() {
    let digit = (bytes[i] - b'0') as u32;
    if i < len - 1 {
      ret = cmp::max(ret, digit * 10 + max_rhs);
    }
    max_rhs = cmp::max(max_rhs, digit);
  }
  return ret;
}

fn main() {
  let stdin: Stdin = io::stdin();
  let mut input_buffer: String = String::new();
  let mut ret : u32 = 0;
  let mut cnt = 0;
  println!("fff");
  loop {
    input_buffer.clear();
    match stdin.read_line(&mut input_buffer) {
      Ok(bytes_read) => {
        if bytes_read == 0 {
          break;
        }
        let trimmed = input_buffer.trim_end();
        let cur = find_joltage(trimmed);
        ret += cur;
        println!("{}> {}", { cnt += 1; cnt }, cur);
      }
      Err(err) => {
        eprintln!("error: {}", err);
      }
    }
  }
  println!("{}", ret);
}

