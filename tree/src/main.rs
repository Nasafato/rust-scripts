use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() {
    let current_dir = env::current_dir().unwrap();
    println!("Current dir: {:?}", current_dir);

    // let mut raw_path_string = String::new();
    // io::stdin()
    //     .read_line(&mut raw_path_string)
    //     .expect("Failed to read line");
    let raw_path_string = "test";

    let mut path = PathBuf::new();
    path.push(current_dir);
    path.push(raw_path_string.trim());

    match tree(path) {
        Ok(()) => println!("Ok"),
        Err(err) => println!("Err: {}", err),
    }
}

/*
[x] Prints out every directory and file under the passed-in path
[x] Print out only the directory or file name, not the full path
[x] Print out the pathname with a certain indentation based on how deep we are
[x] Only print out indentation for indices past the parent's final indentation character

[x] If indented, first non-blank character needs to be:
    [x] If not last-child of parent, '+'
    [x] Else if last-child, '\'

[x] We are running DFS, so we really do need a stack, I need to make sure the order of things is right for
    last and non-last child printing

For any given line, we need:
- Are you the last child?
    - We get length of dir_entry results and ask if you are the last index, i.e. index = len - 1
- What deepness are you at?
    - number of blanks is deepness minus 1
    - number of '-' is simply deepness times amount_per_step

[x] If line indented and ancestors is not the not last-child of its parent, character at which ancestor's indentation starts
    should be '|'
For any given line, we need:
- Are you the last child to be displayed? (not now)
- How many ancestors do you have? - this is your deepness
[Tree character sets](https://unix.stackexchange.com/questions/127063/tree-command-output-with-pure-7-bit-ascii-output)

[x] Sort the results of reading a directory alphabetically
[x] Handle the last child of last child of last child edge case for indenting
- This means all your ancestors are last children, and we can add this to each entry, I think
    - Something like `ancestors_are_last_children` - if true, then all your ancestors are last children
      so you don't print the '|' at your ancestor indices

[] Better error-handlng when the tree encounters non-Unicode filenames
[] List files in current directory when there are no arguments
[] Take in a list of directory arguments and prints out the tree one-by-one
[] Return total number of files and/or directories 
*/

enum Charset {
    Ascii,
    Fancy,
}

#[derive(Clone, Debug)]
struct Line {
    depth: usize,
    is_last_child: bool,
    ancestors_are_last_children: bool,
    path: PathBuf,
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

fn tree(path: PathBuf) -> Result<(), io::Error> {
    let mut stack = Vec::new();
    stack.push(Line::new(0, true, true, path));
    while stack.len() > 0 {
        let line = stack.pop().unwrap();
        line.display();
        if !line.path.is_dir() {
            continue;
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
