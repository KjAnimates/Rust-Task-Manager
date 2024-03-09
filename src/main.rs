mod task;
use task::Task;

fn main() {
    let t: Task = Task { name: "Task".to_string() };
    t.display();
}