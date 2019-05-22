use std::env;
use tree;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let config = match tree::parse_config(&args) {
        Ok(config) => config,
        Err(string) => panic!(string),
    };

    match tree::run(config) {
        Ok(output) => output.print_count(),
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
[x] Take in a list of directory arguments and prints out the tree one-by-one
[] Return total number of files and/or directories
*/
