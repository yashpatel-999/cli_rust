use std::fmt;

/// Custom error types for the file management system
#[derive(Debug, Clone)]
pub enum FileError {
    NotFound(String),
    AlreadyExists(String),
    InvalidInput(String),
    AccessDenied(String),
    EmptyContent,
    InvalidId(u32),
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::NotFound(name) => write!(f, "File '{}' not found", name),
            FileError::AlreadyExists(name) => write!(f, "File '{}' already exists", name),
            FileError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            FileError::AccessDenied(msg) => write!(f, "Access denied: {}", msg),
            FileError::EmptyContent => write!(f, "Cannot create file with empty content"),
            FileError::InvalidId(id) => write!(f, "Invalid file ID: {}", id),
        }
    }
}

impl std::error::Error for FileError {}

/// Result type alias for file operations
pub type FileResult<T> = Result<T, FileError>;
