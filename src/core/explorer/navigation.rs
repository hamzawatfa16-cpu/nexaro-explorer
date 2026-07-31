use std::path::PathBuf;

pub struct Navigation {
    current: PathBuf,
    back: Vec<PathBuf>,
    forward: Vec<PathBuf>,
}

impl Navigation {
    pub fn new(start: PathBuf) -> Self {
        Self {
            current: start,
            back: Vec::new(),
            forward: Vec::new(),
        }
    }

    pub fn current(&self) -> &PathBuf {
        &self.current
    }

    pub fn go_to(&mut self, path: PathBuf) {
        self.back.push(self.current.clone());
        self.current = path;
        self.forward.clear();
    }

    pub fn back(&mut self) {
        if let Some(previous) = self.back.pop() {
            self.forward.push(self.current.clone());
            self.current = previous;
        }
    }

    pub fn forward(&mut self) {
        if let Some(next) = self.forward.pop() {
            self.back.push(self.current.clone());
            self.current = next;
        }
    }
}