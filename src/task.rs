pub struct Task {
    pub name: String
}

impl Task {
    pub fn display(&self) {
        println!("{}", self.name);
    }
    pub fn new(name: &str) -> Task {
        Task { name: name.to_string() }
    }
}