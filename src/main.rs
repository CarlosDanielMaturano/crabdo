use serde::{Deserialize, Serialize};
use std::{env, usize};
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
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

fn get_todos() -> Result<Todos, std::io::Error> {
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

fn save_todo_list(todo_list: Todos) -> Result<(), std::io::Error> {
    let serialized = serde_json::to_string_pretty(&todo_list)?;
    std::fs::write("todos.json", serialized)?;
    Ok(())
}

fn create_new_todo(title: String) -> Result<(), std::io::Error> {
    let todo = Todo::new(title);
    let mut todo_list = get_todos().unwrap_or(Vec::new());
    todo_list.push(todo);
    save_todo_list(todo_list)?;
    Ok(())
}

fn set_todo_state(index: usize, state: bool) -> Result<(), std::io::Error> {
    let mut todo_list = get_todos().unwrap_or(Vec::new());
    if todo_list.is_empty() {
        panic!("No todos to complete");
    }
    todo_list[index].completed = state;
    save_todo_list(todo_list)?;
    Ok(())
}

fn delete_todo(index: usize) -> Result<(), std::io::Error> {
    let mut todo_list = get_todos()?;
    todo_list.remove(index);
    save_todo_list(todo_list)?;
    Ok(())
}

fn show_todos() {
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


fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let main_command: &str = &args[0];

    match main_command {
        "new" => {
            let todo_title: String = args[1].to_owned();
            println!("Creating new todo: {}", &todo_title);
            if let Err(err) = create_new_todo(todo_title) {
                eprintln!("{err}")
            };
        }
        "show" => show_todos(),
        "c" => set_todo_state(0, false).unwrap(),
        "d" => delete_todo(0).unwrap(),
        _ => {
            panic!("unreachable command")
        }
    }
}
