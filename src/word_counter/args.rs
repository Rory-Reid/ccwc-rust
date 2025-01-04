use clap::Parser;
#[derive(Parser)]
#[command()]
pub struct Args {
  #[arg()]
  pub file: String,

  #[arg(short = 'c')]
  count_bytes: bool,

  #[arg(short = 'l')]
  count_lines: bool,

  #[arg(short = 'w')]
  count_words: bool,

  #[arg(short = 'm')]
  count_characters : bool,
}

impl Args {
  pub fn char_output_type(&self) -> OutputType {
    if self.count_characters {
      return OutputType::Characters
    } else if self.count_bytes {
      return OutputType::Bytes
    } else if self.is_default_option() {
      return OutputType::Bytes
    }

    OutputType::None
  }

  pub fn should_output_lines(&self) -> bool {
    self.count_lines || self.is_default_option()
  }

  pub fn should_output_words(&self) -> bool {
    self.count_words || self.is_default_option()
  }

  fn is_default_option(&self) -> bool {
    // If no flag specified, is running in default mode
    !self.count_lines && !self.count_words && !self.count_characters && !self.count_bytes
  }
}

pub enum OutputType {
  Bytes,
  Characters,
  None
}

#[cfg(test)]
mod tests {
  use clap::Parser;
  use crate::word_counter::args::{Args, OutputType};

  #[test]
  fn parses_filename() {
    let args = Args::parse_from(["ccwc", "test_file.txt"]);
    assert_eq!(args.file, "test_file.txt");
  }

  #[test]
  fn flags_are_false_if_not_specified() {
    let args = Args::parse_from(["ccwc", "some_file.txt"]);
    assert!(!args.count_bytes);
    assert!(!args.count_lines);
    assert!(!args.count_words);
    assert!(!args.count_characters);
  }

  #[test]
  fn can_set_count_bytes_flag() {
    let args = Args::parse_from(["ccwc", "-c", "some_file.txt"]);
    assert!(args.count_bytes);
  }

  #[test]
  fn can_set_count_lines_flag() {
    let args = Args::parse_from(["ccwc", "-l", "some_file.txt"]);
    assert!(args.count_lines);
  }

  #[test]
  fn can_set_count_words_flag() {
    let args = Args::parse_from(["ccwc", "-w", "some_file.txt"]);
    assert!(args.count_words);
  }

  #[test]
  fn can_set_count_characters_flag() {
    let args = Args::parse_from(["ccwc", "-m", "some_file.txt"]);
    assert!(args.count_characters);
  }

  #[test]
  fn can_set_all_flags() {
    let args = Args::parse_from(["ccwc", "-clwm", "some_file.txt"]);
    assert!(args.count_lines);
    assert!(args.count_words);
    assert!(args.count_characters);
    assert!(args.count_bytes);
  }

  #[test]
  fn default_output_applied_when_no_flags_set() {
    let args = Args { file: "".to_string(), count_bytes: false, count_lines: false, count_words: false, count_characters: false  };
    assert!(args.should_output_lines());
    assert!(args.should_output_words());
    assert!(matches!(args.char_output_type(), OutputType::Bytes));
  }

  #[test]
  fn count_bytes_only() {
    let args = Args { file: "".to_string(), count_bytes: true, count_lines: false, count_words: false, count_characters: false  };
    assert!(!args.should_output_lines());
    assert!(!args.should_output_words());
    assert!(matches!(args.char_output_type(), OutputType::Bytes));
  }

  #[test]
  fn count_lines_only() {
    let args = Args { file: "".to_string(), count_bytes: false, count_lines: true, count_words: false, count_characters: false  };
    assert!(args.should_output_lines());
    assert!(!args.should_output_words());
    assert!(matches!(args.char_output_type(), OutputType::None));
  }

  #[test]
  fn count_words_only() {
    let args = Args { file: "".to_string(), count_bytes: false, count_lines: false, count_words: true, count_characters: false  };
    assert!(!args.should_output_lines());
    assert!(args.should_output_words());
    assert!(matches!(args.char_output_type(), OutputType::None));
  }

  #[test]
  fn count_characters_only() {
    let args = Args { file: "".to_string(), count_bytes: false, count_lines: false, count_words: false, count_characters: true  };
    assert!(!args.should_output_lines());
    assert!(!args.should_output_words());
    assert!(matches!(args.char_output_type(), OutputType::Characters));
  }

  #[test]
  fn count_characters_overrides_count_bytes() {
    let args = Args { file: "".to_string(), count_bytes: true, count_lines: false, count_words: false, count_characters: true  };
    assert!(matches!(args.char_output_type(), OutputType::Characters));
  }
}