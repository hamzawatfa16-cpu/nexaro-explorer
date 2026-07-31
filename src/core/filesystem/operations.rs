use std::fs;
use crate::core::error::NexaroError;
use std::path::Path;
use crate::models::operation_result::OperationResult;


pub fn create_folder(
    path: &Path,
) -> Result<OperationResult, NexaroError>{
    fs::create_dir(path)?;

Ok(OperationResult {
    success: true,
    message: "Folder created successfully".to_string(),
})
}

pub fn delete(
    path: &Path,
) -> Result<OperationResult, NexaroError> {
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }

    Ok(OperationResult {
        success: true,
        message: "Deleted successfully".to_string(),
    })
}

pub fn rename(
    old_path: &Path,
    new_path: &Path,
) -> Result<OperationResult, NexaroError> {
    fs::rename(old_path, new_path)?;

    Ok(OperationResult {
        success: true,
        message: "Renamed successfully".to_string(),
    })
}

pub fn copy(
    source: &Path,
    destination: &Path,
) -> Result<OperationResult, NexaroError> {
    if source.is_file() {
        fs::copy(source, destination)?;
    } else if source.is_dir() {
        fs::create_dir_all(destination)?;
    }

    Ok(OperationResult {
        success: true,
        message: "Copied successfully".to_string(),
    })
}

pub fn move_item(
    source: &Path,
    destination: &Path,
) -> Result<OperationResult, NexaroError> {
    fs::rename(source, destination)?;

    Ok(OperationResult {
        success: true,
        message: "Moved successfully".to_string(),
    })
}
