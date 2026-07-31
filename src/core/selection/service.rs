use std::path::PathBuf;


pub struct SelectionService {
    selected: Vec<PathBuf>,
}


impl SelectionService {

    pub fn new() -> Self {
        Self {
            selected: Vec::new(),
        }
    }


    pub fn select(
        &mut self,
        path: PathBuf,
    ) {
        if !self.selected.contains(&path) {
            self.selected.push(path);
        }
    }


    pub fn deselect(
        &mut self,
        path: &PathBuf,
    ) {
        self.selected
            .retain(|item| item != path);
    }


    pub fn clear(
        &mut self,
    ) {
        self.selected.clear();
    }


    pub fn selected(
        &self,
    ) -> &[PathBuf] {
        &self.selected
    }


    pub fn select_all(
        &mut self,
        files: &[PathBuf],
    ) {
        self.selected = files.to_vec();
    }
}