use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    levels: Option<usize>,
    paths: Vec<PathBuf>,
}

pub fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    let mut paths = Vec::new();
    let mut levels: Option<usize> = None;
    let mut is_parsing_level = false;
    for arg in args {
        if arg == "-L" {
            is_parsing_level = true;
        } else if is_parsing_level {
            levels = Some(parse_int(&arg)?);
        } else {
            paths.push(PathBuf::from(arg));
        }
    }
    let config = Config { levels, paths };
    Ok(config)
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

pub fn run(config: Config) -> Result<(), io::Error> {
    let mut stack = Vec::new();
    for path in config.paths {
        stack.push(Line::new(0, true, true, path));
    }
    while stack.len() > 0 {
        let line = stack.pop().unwrap();
        line.display();
        if !line.path.is_dir() {
            continue;
        }
        if let Some(levels) = config.levels {
            if levels == line.depth {
                continue;
            }
        }
        let mut paths: Vec<_> = fs::read_dir(&line.path)?.map(|r| r.unwrap()).collect();
        paths.sort_by_key(|dir| dir.path());
        for (i, entry) in paths.into_iter().enumerate() {
            stack.push(Line::new(
                line.depth + 1,
                i == 0,
                line.ancestors_are_last_children && line.is_last_child,
                entry.path(),
            ));
        }
    }
    Ok(())
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

    /// Displays the tree using the Ascii charset
    pub fn display(&self) {
        let indent = create_indentation(&self, 4);
        println!(
            "{}{}",
            indent,
            &self.path.file_name().unwrap().to_str().unwrap()
        );
    }

    /// Lets you specify a charset to display the tree with
    pub fn display_with_charset(&self, charset: Charset) {
        let indent = create_indentation(&self, 4);
        println!("{}{:?}", indent, &self.path.file_name().unwrap());
    }
}


pub enum Charset {
    Ascii,
    Fancy,
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
