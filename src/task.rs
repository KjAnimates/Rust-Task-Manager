pub struct Task {
    pub name: String
}

impl Task {
    pub fn display(&self) {
        println!("{}", self.name);
    }
}