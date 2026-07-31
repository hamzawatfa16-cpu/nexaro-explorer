use std::path::PathBuf;


#[derive(Debug, Clone)]
pub enum ExplorerLocation {

    Folder(PathBuf),

    ThisPc,

    QuickAccess,

    Network,
}