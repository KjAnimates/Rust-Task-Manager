mod task;
use task::Task;
use std::io;

/// Creates a new task struct.
fn new_task(name: &str) -> Task {
    Task { name: name.to_string() }
}

fn main() {
    let mut tasks: Vec<Task> = vec![];
    let t1 = new_task("test");

    // Ask for name.
    let mut username = String::new();
    let _ = io::stdin().read_line(&mut username);
    
    tasks.push(t1);

    for task in tasks {
        println!("{}", &task.name);
    }
}