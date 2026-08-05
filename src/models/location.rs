use std::path::PathBuf;


#[derive(Debug, Clone)]
pub enum ExplorerLocation {

    Folder(PathBuf),

    Home,

    ThisPc,

    QuickAccess,

    Network,
}