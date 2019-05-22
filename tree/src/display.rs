use std::path::PathBuf;

use crate::config::Config;

pub enum Charset {
  Ascii,
  Fancy,
}

#[derive(Clone, Debug)]
pub struct Line {
  pub depth: usize,
  pub is_last_child: bool,
  pub ancestors_are_last_children: bool,
  pub path: PathBuf,
}

impl Line {
  pub fn new(
    depth: usize,
    is_last_child: bool,
    ancestors_are_last_children: bool,
    path: PathBuf,
  ) -> Line {
    Line {
      depth: depth,
      is_last_child: is_last_child,
      ancestors_are_last_children: ancestors_are_last_children,
      path: path,
    }
  }
}

/// Displays the tree using the Ascii charset
pub fn display(line: &Line, config: &Config) {
  let indent = create_indentation(line, 4);
  let filename = match line.path.file_name() {
    Some(filename) => filename.to_string_lossy(),
    None => line.path.to_string_lossy(),
  };

  let should_print = match config.only_show_directories {
    true => line.path.is_dir(),
    false => true,
  };
  if should_print {
    println!("{}{}", indent, filename);
  }
}

/// Lets you specify a charset to display the tree with
pub fn display_with_charset(line: &Line, charset: Charset) {
  let indent = create_indentation(line, 4);
  println!("{}{:?}", indent, line.path.file_name().unwrap());
}

fn create_indentation(line: &Line, amount_per_step: usize) -> String {
  let mut indent = "".to_string();
  if line.depth == 0 {
    return indent;
  }
  indent.push_str(
    create_ancestor_indent(
      line.depth - 1,
      line.ancestors_are_last_children,
      amount_per_step,
    )
    .as_ref(),
  );
  if line.is_last_child {
    indent.push_str("\\");
  } else {
    indent.push_str("+")
  }
  while indent.len() < line.depth * amount_per_step {
    indent.push_str("-");
  }
  indent
}

fn create_ancestor_indent(
  steps: usize,
  ancestors_are_last_children: bool,
  amount_per_step: usize,
) -> String {
  let mut ancestor_indent = "".to_string();
  while ancestor_indent.len() < steps * amount_per_step {
    let mut step = "".to_string();
    if ancestors_are_last_children {
      step.push_str(" ");
    } else {
      step.push_str("|");
    }
    while step.len() < amount_per_step {
      step.push_str(" ");
    }
    ancestor_indent.push_str(&step)
  }
  ancestor_indent
}
