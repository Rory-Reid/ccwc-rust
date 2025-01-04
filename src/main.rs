use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use clap::Parser;

#[derive(Parser)]
#[command()]
struct Args {
  #[arg()]
  file: String,

  #[arg(short = 'c')]
  count_bytes: bool,

  #[arg(short = 'l')]
  count_lines: bool,

  #[arg(short = 'w')]
  count_words: bool,

  #[arg(short = 'm')]
  count_characters : bool,
}

fn main() {
  let args = Args::parse();

  match args.count_bytes || args.count_lines || args.count_words || args.count_characters {
    true => {}
    false => panic!("Current scope only supports -l and/or -c and/or -w and/or -m flags. At least one must be present.")
  }

  let mut file = File::open(&args.file).unwrap(); // Todo - handle error and don't just unwrap
  file.seek(SeekFrom::Start(0)).unwrap();

  let mut bytes_read : usize;
  let mut total_bytes : usize = 0;
  let mut lines : i64 = 0;

  let mut reading_word = false;
  let mut words : i64 = 0;

  let mut characters : i64 = 0;

  // This essentially creates a fixed-length vector on the heap pre-allocated so that we can use it
  // as a buffer. I didn't use an array because it overflows the stack, and I had issues with that
  // when trying to allocate a boxed array too. The vector is pre-initialised to "full" capacity
  // because the file.read() function will fill a buffer to current capacity, i.e. a vec of capacity
  // 100 but current size 0 will result in reading 0 bytes. See https://stackoverflow.com/questions/41710952/allocate-array-onto-heap-with-size-known-at-runtime
  const MAX_BUF_SIZE: usize = 104_857_600; // 100mb
  let file_size = file.seek(SeekFrom::End(0)).unwrap() as usize;
  file.seek(SeekFrom::Start(0)).unwrap();
  let buf_size = if file_size > MAX_BUF_SIZE {MAX_BUF_SIZE} else {file_size};
  let mut buffer = vec![0; buf_size];
  loop {
    bytes_read = file.read(&mut buffer).unwrap(); // Todo - handle error and don't just unwrap
    if bytes_read == 0 {
      break;
    }

    let mut adjust = 0;
    for utf8_chunk in buffer[0..bytes_read].utf8_chunks() {
      let valid = utf8_chunk.valid();
      let invalid = utf8_chunk.invalid();
      adjust = invalid.len();

      for character in valid.chars() {
        characters += 1;
        if character.is_whitespace() {
          if character == '\n' {
            lines += 1;
          }

          words += if reading_word {1} else {0};
          reading_word = false;
        } else {
          reading_word = true;
        }
      }
    }

    if adjust == bytes_read {
      // File has ended with invalid bytes - exit loop
      total_bytes += bytes_read;
      break;
    }

    total_bytes += bytes_read - adjust; // adjust by invalid bytes so we can re-read them
    if bytes_read < buf_size {
      // end of file
      break;
    }

    _ = file.seek(SeekFrom::Start(total_bytes as u64)).unwrap() // Todo - error handling
  }

  // Applies -m if specified, ignoring -c, otherwise applies -c if specified
  let bytes_output = match args.count_characters {
    true => format!("{: >8}", characters),
    false => match args.count_bytes {
      true => format!("{: >8}", total_bytes),
      false => String::new(),
    }
  };

  let lines_output = match args.count_lines {
    true => format!("{: >8}", lines),
    false => String::new(),
  };

  let words_output = match args.count_words {
    true => format!("{: >8}", words),
    false => String::new(),
  };

  println!("{}{}{} {}", lines_output, words_output, bytes_output, args.file);
}
