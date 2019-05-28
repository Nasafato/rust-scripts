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
    while let Some(line) = stack.pop() {
        if line.path.is_dir() {
            output.increment_directories();
        } else {
            output.increment_files();
        }
        display::display(&line, &args.config);
        if !line.path.is_dir() {
            continue;
        }

        // https://doc.rust-lang.org/rust-by-example/error/iter_result.html#fail-the-entire-operation-with-collect
        let paths: Result<Vec<_>, io::Error> = fs::read_dir(&line.path)?.collect();
        let mut paths = match paths {
            Ok(paths) => paths,
            Err(err) => return Err(err),
        };
        let new_depth = line.depth + 1;
        let config = &args.config;
        paths.sort_by_key(|dir| dir.path());
        for (i, entry) in paths
            .into_iter()
            .filter(|e| {
                if let Some(max_depth) = config.levels {
                    if new_depth > max_depth {
                        return false;
                    }
                }
                if config.print_hidden {
                    return true;
                }
                (!config.print_hidden && !utils::is_hidden(&e.path()))
            })
            .enumerate()
        {
            let path = entry.path();
            stack.push(Line::new(
                new_depth,
                i == 0,
                line.ancestors_are_last_children && line.is_last_child,
                path,
            ));
        }
    }
    Ok(output)
}
