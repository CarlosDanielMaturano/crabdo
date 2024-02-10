# CRABDO

![Static Badge](https://img.shields.io/badge/cargo-1.74.1%20-blue)
![Static Badge](https://img.shields.io/badge/LICENSE-MIT-green)

## A way of manage todos with the command line writen in rust

# Usage

```bash
    git clone  https://carlosdanielmaturano/crabdo
    cd crabdo
    cargo run <OPTION>
```

<p>Or build a standalone binary</p>

```bash
    cargo build --release
    ./target/release/crabdo
```

# Getting Help

<p>if you type a unknown a command or run with the "help" option, the program will display some help information</p>

```bash
 l | list => list the todos in the current directory
 n | new <TITLE> => create a new todo with the given title
 c | complete <ID> => mark the todo with the given id as completed
 u | uncomplete <ID> => mark the todo with the given id as uncompleted
 d | delete <ID> => delete the todo with the given id
```
