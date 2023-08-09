pub struct Task {
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(description: &str) -> Self {
        Self {
            description: description.into(),
            completed: false,
        }
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
    }
}
