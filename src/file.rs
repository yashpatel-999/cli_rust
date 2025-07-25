use crate::error::{FileError, FileResult};
use std::fmt;

/// Represents a file in memory
#[derive(Debug, Clone)]
pub struct File {
    pub id: u32,
    pub name: String,
    pub content: String,
    pub size: usize,
    pub created_at: std::time::SystemTime,
}

impl File {
    /// Creates a new file with the given name and content
    pub fn new(id: u32, name: String, content: String) -> FileResult<Self> {
        if name.trim().is_empty() {
            return Err(FileError::InvalidInput("File name cannot be empty".to_string()));
        }
        
        let size = content.len();
        let created_at = std::time::SystemTime::now();
        
        Ok(File {
            id,
            name,
            content,
            size,
            created_at,
        })
    }

    /// Updates the file content
    pub fn write_content(&mut self, content: String) {
        self.content = content;
        self.size = self.content.len();
    }

    /// Gets a preview of the file content (first 50 characters)
    pub fn preview(&self) -> &str {
        if self.content.len() > 50 {
            &self.content[..50]
        } else {
            &self.content
        }
    }

    /// Gets the file extension
    pub fn extension(&self) -> Option<&str> {
        self.name.split('.').last()
    }
}

/// Trait for displaying file information
pub trait FileDisplay {
    fn display_summary(&self) -> String;
    fn display_detailed(&self) -> String;
}

impl FileDisplay for File {
    fn display_summary(&self) -> String {
        format!("[{}] {} ({} bytes)", self.id, self.name, self.size)
    }

    fn display_detailed(&self) -> String {
        let elapsed = self.created_at.elapsed()
            .unwrap_or_else(|_| std::time::Duration::new(0, 0));
        
        format!(
            "ID: {}\nName: {}\nSize: {} bytes\nCreated: {:?} ago\nPreview: {}{}",
            self.id,
            self.name,
            self.size,
            elapsed,
            self.preview(),
            if self.content.len() > 50 { "..." } else { "" }
        )
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_summary())
    }
}

/// In-memory file system manager
pub struct FileSystem {
    files: Vec<File>,
    next_id: u32,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            files: Vec::new(),
            next_id: 1,
        }
    }

    /// Creates a new file
    pub fn create_file(&mut self, name: String, content: String) -> FileResult<u32> {
        // Check if file already exists
        if self.files.iter().any(|f| f.name == name) {
            return Err(FileError::AlreadyExists(name));
        }

        let id = self.next_id;
        let file = File::new(id, name, content)?;
        self.files.push(file);
        self.next_id += 1;
        Ok(id)
    }

    /// Writes content to an existing file
    pub fn write_file(&mut self, name: &str, content: String) -> FileResult<()> {
        match self.files.iter_mut().find(|f| f.name == name) {
            Some(file) => {
                file.write_content(content);
                Ok(())
            }
            None => Err(FileError::NotFound(name.to_string())),
        }
    }

    /// Reads a file's content
    pub fn read_file(&self, name: &str) -> FileResult<&str> {
        match self.files.iter().find(|f| f.name == name) {
            Some(file) => Ok(&file.content),
            None => Err(FileError::NotFound(name.to_string())),
        }
    }

    /// Gets a file by name
    pub fn get_file(&self, name: &str) -> FileResult<&File> {
        match self.files.iter().find(|f| f.name == name) {
            Some(file) => Ok(file),
            None => Err(FileError::NotFound(name.to_string())),
        }
    }

    /// Gets a file by ID
    pub fn get_file_by_id(&self, id: u32) -> FileResult<&File> {
        match self.files.iter().find(|f| f.id == id) {
            Some(file) => Ok(file),
            None => Err(FileError::InvalidId(id)),
        }
    }

    /// Lists all files
    pub fn list_files(&self) -> &[File] {
        &self.files
    }

    /// Deletes a file by name
    pub fn delete_file(&mut self, name: &str) -> FileResult<()> {
        match self.files.iter().position(|f| f.name == name) {
            Some(index) => {
                self.files.remove(index);
                Ok(())
            }
            None => Err(FileError::NotFound(name.to_string())),
        }
    }

    /// Deletes a file by ID
    pub fn delete_file_by_id(&mut self, id: u32) -> FileResult<()> {
        match self.files.iter().position(|f| f.id == id) {
            Some(index) => {
                self.files.remove(index);
                Ok(())
            }
            None => Err(FileError::InvalidId(id)),
        }
    }

    /// Gets the total number of files
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Gets the total size of all files
    pub fn total_size(&self) -> usize {
        self.files.iter().map(|f| f.size).sum()
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        Self::new()
    }
}
