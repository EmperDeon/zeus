use std::io;
use std::io::Write;

pub mod git;
pub mod logger;
pub mod services;
pub mod config;

pub fn ask(question: &str) -> bool {
  print!("\n");

  loop {
    print!("{} (y/n): ", question);
    io::stdout().flush().unwrap();

    let response: char = text_io::read!("{}\n");

    if response == 'y' || response == 'Y' {
      return true
    }

    if response == 'n' || response == 'N' {
      return false
    }
  }
}
