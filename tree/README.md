# rust-scripts/tree

I am attempting to rebuild Unix's [tree](http://mama.indstate.edu/users/ice/tree/) command.

### We have the following functionality implemented:
> With no arguments, tree lists the files in the current directory. 

> When directory arguments are given, tree lists all the files and/or directories found in the given directories each in turn. 

> Upon completion of listing all files/directories found, tree returns the total number of files and/or directories listed.

### Plus these flags and options:
| Flag                | Description                                                                          |
| ------------------- | ------------------------------------------------------------------------------------ |
| `-a`                | Prints hidden files                                                                  |
| `-d`                | Lists directories only                                                               |
| `-L level`          | Max display depth of the tree                                                        |
| `--charset charset` | Charset to use when printing, I only have the default plus `ascii` as options so far |
