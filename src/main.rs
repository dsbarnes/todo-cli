// Why not std::io ??
// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
use std::env;

struct TodoItem{
    name: String,
    completed: char,
}

impl TodoItem{
    fn new(name: String) -> TodoItem {
        TodoItem{
            name,
            completed: ' ',
        }
    }
}

struct TodoList{
    todos: Vec<TodoItem>,
}

impl TodoList{
    fn new() -> TodoList{
        TodoList{ todos: Vec::new(), }
    }

    fn add(&mut self, todo: TodoItem){
        self.todos.push(todo);
    }

    fn remove(&mut self, todo_index: usize){
        self.todos.remove(todo_index);
    }

    fn complete(&mut self, todo_index: usize){
        self.todos[todo_index].completed = 'x';
    }

    fn incomplete(&mut self, todo_index: usize){
        self.todos[todo_index].completed = ' ';
    }

    fn print(&self){
        for todo in &self.todos{
            println!("[ {} ] - {}", todo.completed, todo.name);
        }
    }
}

enum Command{
    Get,
    Add(String),
    Remove(usize),
    Complete(usize),
    Incomplete(usize),
}


fn main() {
    // Collect the arguments:
    let args: Vec<String> = env::args().collect();
    // Init a todo list:
    let todo_list = TodoList::new();
    // Match the command that was given:
    let command = match args[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(args[2].clone()),
        "remove" => Command::Remove(args[2].parse().expect("")),
        "complete" => Command::Complete(args[2].parse().expect("")),
        "incomplete" => Command::Incomplete(args[2].parse().expect("")),
        _ => panic!("Must provide a valid command\nget, add, remove, complete, incomplete"),
    };
    // Match the command to its functionality:
    // Output a success of failure message (and exit):
}





