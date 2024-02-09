use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Read};
use std::{env, usize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

fn create_new_todo(title: &str) -> Result<(), Error> {
    let todo = Todo::new(title.to_string());
    let mut todo_list = get_todos().unwrap_or(Vec::new());
    if todo_list.contains(&todo) {
        return Err(std::io::Error::new(
            ErrorKind::AlreadyExists,
            "Todo already exists on the todo list",
        ));
    }
    todo_list.push(todo);
    save_todo_list(todo_list)?;
    println!("Sucessfully created a new todo: {title}");
    Ok(())
}

fn set_todo_state(index: usize, state: bool) -> Result<(), Error> {
    let mut todo_list = get_todos().unwrap_or(Vec::new());
    if todo_list.is_empty() {
        return Err(Error::new(ErrorKind::Other, "The todo list is empty"));
    }
    todo_list[index].completed = state;
    save_todo_list(todo_list)?;
    Ok(())
}

fn delete_todo(index: usize) -> Result<(), Error> {
    let mut todo_list = get_todos()?;
    let todo = todo_list.get(index);
    let Some(todo) = todo else {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Invalid todo at position {index}"),
        ));
    };
    if !todo.completed {
        return Err(Error::new(ErrorKind::Other, "Todo is not yet completed"));
    }
    todo_list.remove(index);
    save_todo_list(todo_list)?;
    Ok(())
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
    println!("No help, No help for you >:[")
}

fn get_index_from_arg(args: &Vec<String>) -> usize {
    if args.len() <= 1 {
        println!("Error: No todo ID has been provided");
        std::process::exit(0)
    }
    let result = args[1].trim().parse::<usize>();
    if let Err(err) = result {
        println!("A error ocurred while parsing argv 1. Err: {err}");
        std::process::exit(0);
    }
    let mut index: usize = result.unwrap();

    if index > 0 {
        index -= 1
    }

    index
}

fn exit_with_error(err_message: String) {
    println!("{err_message}");
    std::process::exit(0);
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
            if let Err(err) = create_new_todo(title) {
                exit_with_error(format!(
                    "A error ocurred during the creation of a todo: {err}"
                ));
            };
        }
        "complete" | "c" => {
            let index: usize = get_index_from_arg(&args);
            if let Err(err) = set_todo_state(index, true) {
                exit_with_error(format!(
                    "A error ocurred while setting the state of the todo. Err: {err}"
                ));
            };
            println!("yay :D");
        }
        "delete" | "d" => {
            let index: usize = get_index_from_arg(&args);
            if let Err(err) = delete_todo(index) {
                exit_with_error(format!(
                    "A error ocurred while deleting the todo. Err: {err}"
                ));
            }
            println!("yay :D");
        }
        _ => show_help(),
    }
}

fn main() {
    cli();
}
