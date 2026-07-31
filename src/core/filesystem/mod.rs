pub mod operations;
pub mod reader;

use std::path::Path;

use crate::core::error::NexaroError;
use crate::models::file_info::FileInfo;
use crate::models::operation_result::OperationResult;

pub struct FileSystem;

impl FileSystem {
    pub fn new() -> Self {
        Self
    }

    pub fn list_directory(
        &self,
        path: &Path,
    ) -> Result<Vec<FileInfo>, NexaroError> {
        reader::read_directory(path)
    }

    pub fn create_folder(
        &self,
        path: &Path,
    ) -> Result<OperationResult, NexaroError> {
        Ok(operations::create_folder(path)?)
    }

    pub fn delete(
        &self,
        path: &Path,
    ) -> Result<OperationResult, NexaroError> {
        Ok(operations::delete(path)?)
    }

    pub fn rename(
        &self,
        old_path: &Path,
        new_path: &Path,
    ) -> Result<OperationResult, NexaroError> {
        Ok(operations::rename(old_path, new_path)?)
    }

    pub fn copy(
        &self,
        source: &Path,
        destination: &Path,
    ) -> Result<OperationResult, NexaroError> {
        Ok(operations::copy(source, destination)?)
    }

    pub fn move_item(
        &self,
        source: &Path,
        destination: &Path,
    ) -> Result<OperationResult, NexaroError> {
        Ok(operations::move_item(source, destination)?)
    }
}