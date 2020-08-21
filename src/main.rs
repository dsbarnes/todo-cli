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
        TodoList{
            todos: Vec::new(),
        }
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
}

enum Command{
    Get,
    Add(String),
    Remove(usize),
    Complete(usize),
    Incomplete(usize),
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let todo = TodoItem::new(args[1].clone().to_string());

}





