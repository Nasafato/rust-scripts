use std::fs;
use std::io;
use std::path::PathBuf;
mod config;
mod display;
mod output;
mod utils;
pub use config::Args;
use display::Line;
use output::Output;

pub fn run(args: Args) -> Result<Output, io::Error> {
    let mut stack = Vec::new();
    let mut output = Output::new(args.paths.len());
    initialize_stack(args.paths, &mut stack);
    while let Some(line) = stack.pop() {
        increment_output(&line, &mut output);
        display::display(&line, &args.config);
        if !line.path.is_dir() {
            continue;
        }

        add_children(&line, &args.config, &mut stack)?;
    }
    Ok(output)
}

fn initialize_stack(paths: Vec<PathBuf>, stack: &mut Vec<display::Line>) {
    for path in paths {
        stack.push(Line::new(0, true, true, path));
    }
}

fn increment_output(line: &display::Line, output: &mut Output) {
    if line.path.is_dir() {
        output.increment_directories();
    } else {
        output.increment_files();
    }
}

fn add_children(
    line: &display::Line,
    config: &config::Config,
    stack: &mut Vec<display::Line>,
) -> Result<(), io::Error> {
    // https://doc.rust-lang.org/rust-by-example/error/iter_result.html#fail-the-entire-operation-with-collect
    let paths: Result<Vec<fs::DirEntry>, io::Error> = fs::read_dir(&line.path)?.collect();
    let mut paths = paths?;
    let new_depth = line.depth + 1;
    paths.sort_by_key(|dir| dir.path());
    for (i, entry) in filter_children(paths, config, new_depth).enumerate() {
        let path = entry.path();
        stack.push(Line::new(
            new_depth,
            i == 0,
            line.ancestors_are_last_children && line.is_last_child,
            path,
        ));
    }
    Ok(())
}

fn filter_children(
    paths: Vec<fs::DirEntry>,
    config: &config::Config,
    new_depth: usize,
) -> std::iter::Filter<fs::DirEntry> {
    paths.into_iter().filter(|e| {
        // There should be some precedence for these, though it maybe isn't a
        // problem?
        if let Some(max_depth) = config.levels {
            if new_depth > max_depth {
                return false;
            }
        }
        if config.only_show_directories && !e.path().is_dir() {
            return false;
        }
        if config.print_hidden {
            return true;
        }
        (!config.print_hidden && !utils::is_hidden(&e.path()))
    })
}
