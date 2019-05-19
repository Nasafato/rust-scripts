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

*/

fn tree(path: PathBuf) -> Result<(), io::Error> {
    let mut stack = Vec::new();
    stack.push((0, true, true, path));
    while stack.len() > 0 {
        let (deepness, is_last_child, ancestors_are_last_children, path) = stack.pop().unwrap();
        let indent = create_indentation(deepness, is_last_child, ancestors_are_last_children, 4);
        println!("{}{:?}", indent, path.file_name().unwrap());
        if !path.is_dir() {
            continue;
        }
        let mut paths: Vec<_> = fs::read_dir(&path)?.map(|r| r.unwrap()).collect();
        paths.sort_by_key(|dir| dir.path());

        for (i, entry) in paths.into_iter().enumerate() {
            let entry_path = entry.path();
            let is_last_child_to_be_printed = i == 0;
            let ancestors_are_last_children_for_child =
                ancestors_are_last_children && is_last_child;
            stack.push((
                deepness + 1,
                is_last_child_to_be_printed,
                ancestors_are_last_children_for_child,
                entry_path,
            ));
        }
    }
    Ok(())
}

fn create_indentation(
    deepness: usize,
    is_last_child: bool,
    ancestors_are_last_children: bool,
    amount_per_step: usize,
) -> String {
    let mut indent = "".to_string();
    if deepness == 0 {
        return indent;
    }
    indent.push_str(
        create_ancestor_indent(deepness - 1, ancestors_are_last_children, amount_per_step).as_ref(),
    );
    if is_last_child {
        indent.push_str("\\");
    } else {
        indent.push_str("+")
    }
    while indent.len() < deepness * amount_per_step {
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
