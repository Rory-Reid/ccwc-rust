use std::fs::File;
use std::io::{Seek, SeekFrom};
use clap::Parser;

#[derive(Parser)]
#[command()]
struct Args {
  #[arg()]
  file: String,

  #[arg(short = 'c')]
  count_bytes: bool,
}

fn main() {
  let args = Args::parse();

  match args.count_bytes {
    true => {}
    false => panic!("Only the -c flag is implemented currently!")
  }

  let mut file = match File::open(&args.file) {
    Ok(f) => f,
    Err(_) => panic!("File not found") // Todo
  };

  // This is a bit of a shortcut for satisfying the -c flag only currently
  let size = match file.seek(SeekFrom::End(0)) {
    Ok(size) => size,
    Err(_) => panic!("Cannot seek") // Todo
  };

  println!("{} {}", format!("{: >8}", size), args.file);
}
