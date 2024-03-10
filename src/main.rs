mod task;
use task::Task;
use std::io;

fn main() {
    let mut tasks: Vec<Task> = vec![];
    let t1: Task = Task::new("test");

    // Ask for name.
    let mut username = String::new();
    let _ = io::stdin().read_line(&mut username);
    
    tasks.push(t1);

    for task in tasks {
        println!("{}", &task.name);
    }
}