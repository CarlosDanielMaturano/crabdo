use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Read};
use std::{env, usize};

macro_rules! unwrap_or_return_with_message {
    ( $e:expr, $text:tt) => {
        match $e {
            Ok(value) => value,
            Err(err) => return exit_with_error(format!("{}. Err: {}", $text, err)),
        }
    };
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Todo {
    title: String,
    completed: bool,
}

impl Todo {
    pub fn new(title: String) -> Todo {
        Todo {
            title,
            completed: false,
        }
    }
}

type Todos = Vec<Todo>;

fn exit_with_error(err_message: String) {
    println!("{err_message}");
    std::process::exit(0);
}

fn get_todos() -> Result<Todos, Error> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open("todos.json")?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    let todo_list = if file_content.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str::<Todos>(&file_content)?
    };
    Ok(todo_list)
}

fn save_todo_list(todo_list: Todos) -> Result<(), Error> {
    let serialized = serde_json::to_string_pretty(&todo_list)?;
    std::fs::write("todos.json", serialized)?;
    Ok(())
}

fn create_new_todo(title: &str) -> Result<Todo, Error> {
    let todo = Todo::new(title.to_string());
    let mut todo_list = get_todos().unwrap_or(Vec::new());
    if todo_list.contains(&todo) {
        return Err(std::io::Error::new(
            ErrorKind::AlreadyExists,
            "Todo already exists on the todo list",
        ));
    }
    todo_list.push(todo.clone());
    save_todo_list(todo_list)?;
    Ok(todo)
}

fn set_todo_state(index: usize, state: bool) -> Result<Todo, Error> {
    let mut todo_list = get_todos().unwrap_or(Vec::new());
    if todo_list.is_empty() {
        return Err(Error::new(ErrorKind::Other, "The todo list is empty"));
    }
    if let Some(todo) = todo_list.get_mut(index) {
        *todo = Todo { completed: state, ..todo.clone() };
        let todo_clone = todo.clone();
        if let Err(err) = save_todo_list(todo_list) {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Failed at saving the todo list. Err: {err}"),
            ));
        };
        return Ok(todo_clone)
    }
    return Err(Error::new(
        ErrorKind::Other,
        format!("Invalid index at: {index}"),
    ));
}

fn delete_todo(index: usize) -> Result<Todo, Error> {
    let mut todo_list = get_todos()?;
    let todo = todo_list.get(index);
    let Some(todo) = todo else {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Invalid todo at position {index}"),
        ));
    };
    let mut option = String::new();
    if !todo.completed {
        println!("The todo is noy yet completed, continue anyway? [Y/N]: ");
        if let Err(_) = std::io::stdin().read_line(&mut option) {
            println!("A error ocurred while reading the input, assuming N");
        }
        match option.trim().to_uppercase().as_str() {
            "Y" | "YES" => (),
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Todo is not yet completed",
                ))
            }
        };
    }
    let todo = todo_list.remove(index);
    save_todo_list(todo_list)?;
    Ok(todo)
}

fn list_todos() {
    let todo_list = get_todos().unwrap_or(Vec::new());

    if todo_list.is_empty() {
        return;
    }

    todo_list.iter().enumerate().for_each(|(index, todo)| {
        let title = &todo.title;
        let index = index + 1;
        let completed_symbol = if todo.completed { "[X]" } else { "[]" };
        println!("{index}. {completed_symbol} {title}");
    });
}

// TODOOO: Implement the help function
fn show_help() {
    println!(
       " l | list => list the todos\n \
        n | new <TITLE> => create a new todo with the given title\n \
        c | complete <ID> => mark the todo with the given id as completed\n \
        u | uncomplete <ID> => mark the todo with the given id as uncompleted\n \
        d | delete <ID> => delete the todo with the given id\n"
    )
}

fn get_index_from_arg(args: &Vec<String>) -> Result<usize, Error> {
    if args.len() <= 1 {
        return Err(Error::new(
            ErrorKind::Other,
            "Error: No todo ID has been provided",
        ));
    }
    let result = args[1].trim().parse::<usize>();
    if let Err(err) = result {
        return Err(Error::new(
            ErrorKind::Other,
            format!("A error ocurred while parsing argv 1. Err: {err}"),
        ));
    }
    let mut index: usize = result.unwrap();

    if index > 0 {
        index -= 1
    }
    Ok(index)
}

fn cli() {
    let args = env::args().skip(1).collect::<Vec<String>>();

    if args.is_empty() {
        exit_with_error(format!(
            "Error: no arguments provided, please provide at least one argument"
        ));
    }

    let main_command: &str = &args[0];

    match main_command {
        "list" | "l" => list_todos(),
        "new" | "n" => {
            if args.len() <= 1 {
                exit_with_error("Error: no title has been provided!".to_string());
            }
            let title: &str = &args[1];
            let todo = unwrap_or_return_with_message!(
                create_new_todo(title),
                (String::from("Failed to create a new todo"))
            );
            println!("Sucessfully created a new todo: {}", todo.title);
        }
        "complete" | "c" => {
            let index: usize = unwrap_or_return_with_message!(
                get_index_from_arg(&args),
                (String::from("Failed to get the index from given arguments"))
            );
            let todo = unwrap_or_return_with_message!(
                set_todo_state(index, true),
                (String::from("Failed to create a new todo"))
            );
            println!("Sucessfully set the todo {} as completed", todo.title);
        }

        "uncomplete" | "uc" => {
            let index: usize = unwrap_or_return_with_message!(
                get_index_from_arg(&args),
                (String::from("Failed to get the index from given arguments"))
            );
            let todo = unwrap_or_return_with_message!(
                set_todo_state(index, false),
                (String::from("Failed to create a new todo"))
            );
            println!("Sucessfully set the todo {} as uncompleted", todo.title);
        }
        "delete" | "d" => {
            let index: usize = unwrap_or_return_with_message!(
                get_index_from_arg(&args),
                (String::from("Failed to get the index from given arguments"))
            );
            let todo = unwrap_or_return_with_message!(
                delete_todo(index),
                (format!("Failed to delete the todo at index {index}"))
            );
            println!("Sucessfully deleted the todo: {}", todo.title);
        }
        "h" | "help" | _ => show_help(),
    }
}

fn main() {
    cli();
}
