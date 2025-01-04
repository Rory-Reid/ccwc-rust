use clap::Parser;
use crate::word_counter::args::Args;

mod word_counter;

fn main() {
  let args = Args::parse();
  let output = word_counter::count_words(args);
  println!("{}", output);
}
