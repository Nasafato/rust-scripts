use std::fs;
use std::io;
mod config;
mod display;
mod output;
mod utils;
pub use config::Args;
use config::Config;
use display::Line;
use output::Output;

pub fn run(args: Args) -> Result<Output, io::Error> {
    let mut stack = Vec::new();
    let mut output = Output::new(args.paths.len());
    for path in args.paths {
        stack.push(Line::new(0, true, true, path));
    }
    while stack.len() > 0 {
        let line = match stack.pop() {
            Some(line) => line,
            None => return Err(io::Error::new(io::ErrorKind::Other, "Pop failed")),
        };
        match line.path.is_dir() {
            true => output.increment_directories(),
            false => output.increment_files(),
        }
        display::display(&line, &args.config);
        if should_skip(&line, &args.config) {
            continue;
        }

        // https://doc.rust-lang.org/rust-by-example/error/iter_result.html#fail-the-entire-operation-with-collect
        let paths: Result<Vec<_>, io::Error> = fs::read_dir(&line.path)?.collect();
        let mut paths = match paths {
            Ok(paths) => paths,
            Err(err) => return Err(err),
        };
        paths.sort_by_key(|dir| dir.path());
        for (i, entry) in paths.into_iter().enumerate() {
            let path = entry.path();
            if !args.config.print_hidden && utils::is_hidden(&path) {
                continue;
            }
            stack.push(Line::new(
                line.depth + 1,
                i == 0,
                line.ancestors_are_last_children && line.is_last_child,
                path,
            ));
        }
    }
    Ok(output)
}

fn should_skip(line: &Line, config: &Config) -> bool {
    if let Some(levels) = config.levels {
        if levels == line.depth {
            return true;
        }
    }
    if !line.path.is_dir() {
        return true;
    }

    false
}
