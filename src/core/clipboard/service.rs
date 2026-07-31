use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum ClipboardAction {
    Copy,
    Cut,
}

pub struct ClipboardService {
    files: Vec<PathBuf>,
    action: Option<ClipboardAction>,
}

impl ClipboardService {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            action: None,
        }
    }

    pub fn set(
        &mut self,
        files: Vec<PathBuf>,
        action: ClipboardAction,
    ) {
        self.files = files;
        self.action = Some(action);
    }

    pub fn files(&self) -> &[PathBuf] {
        &self.files
    }

    pub fn action(&self) -> Option<&ClipboardAction> {
        self.action.as_ref()
    }

    pub fn clear(&mut self) {
        self.files.clear();
        self.action = None;
    }
}