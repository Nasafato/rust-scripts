use crate::display::Charset;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Args {
  pub paths: Vec<PathBuf>,
  pub config: Config,
}

impl Args {
  pub fn from_args(arguments: &[String]) -> Result<Self, &'static str> {
    let mut args = Args::new();
    let mut is_parsing_level = false;
    let mut is_parsing_charset = false;
    for arg in arguments {
      if arg == "-L" {
        is_parsing_level = true;
      } else if is_parsing_level {
        args.config.levels = Some(parse_int(&arg)?);
        is_parsing_level = false;
      } else if arg == "-d" {
        args.config.only_show_directories = true;
      } else if arg == "-a" {
        args.config.print_hidden = true;
      } else if arg == "--charset" {
        is_parsing_charset = true;
      } else if is_parsing_charset {
        args.config.charset = parse_charset(arg);
        is_parsing_charset = false;
      } else {
        args.paths.push(PathBuf::from(arg));
      }
    }
    if args.paths.len() == 0 {
      args.paths.push(PathBuf::from("."));
    }
    Ok(args)
  }

  pub fn new() -> Self {
    Args {
      config: Config::new(),
      paths: Vec::new(),
    }
  }
}

fn parse_charset(string: &str) -> Charset {
  match string {
    "fancy" => Charset::Fancy,
    "ascii" => Charset::Ascii,
    _ => Charset::Fancy,
  }
}

#[derive(Debug)]
pub struct Config {
  pub levels: Option<usize>,
  pub only_show_directories: bool,
  pub print_hidden: bool,
  pub charset: Charset,
}

impl Config {
  pub fn new() -> Self {
    Config {
      levels: None,
      only_show_directories: false,
      print_hidden: false,
      charset: Charset::Fancy,
    }
  }
}

fn parse_int(string: &str) -> Result<usize, &'static str> {
  let mut values_to_sum = Vec::new();
  let radix: usize = 10;
  for (i, c) in string.chars().rev().enumerate() {
    match c.to_digit(10) {
      Some(num) => {
        let num = num as usize;
        values_to_sum.push(num * radix.pow(i as u32));
      }
      None => {
        return Err("Couldn't parse string");
      }
    }
  }
  let sum = values_to_sum.iter().sum();
  Ok(sum)
}
