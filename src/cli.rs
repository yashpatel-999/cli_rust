use crate::error::{FileError, FileResult};
use crate::file::{FileSystem, FileDisplay};
use std::io::{self, Write};

/// CLI operations enum
#[derive(Debug, Clone)]
pub enum Operation {
    Create,
    Write,
    Read,
    List,
    Delete,
    Info,
    Help,
    Stats,
    Quit,
}

impl Operation {
    /// Parses a command string into an Operation
    pub fn from_str(input: &str) -> FileResult<Self> {
        match input.trim().to_lowercase().as_str() {
            "create" | "c" => Ok(Operation::Create),
            "write" | "w" => Ok(Operation::Write),
            "read" | "r" => Ok(Operation::Read),
            "list" | "l" | "ls" => Ok(Operation::List),
            "delete" | "d" | "del" => Ok(Operation::Delete),
            "info" | "i" => Ok(Operation::Info),
            "help" | "h" | "?" => Ok(Operation::Help),
            "stats" | "s" => Ok(Operation::Stats),
            "quit" | "q" | "exit" => Ok(Operation::Quit),
            _ => Err(FileError::InvalidInput(format!("Unknown command: {}", input))),
        }
    }
}

/// CLI interface for the file management system
pub struct CLI {
    filesystem: FileSystem,
}

impl CLI {
    pub fn new() -> Self {
        CLI {
            filesystem: FileSystem::new(),
        }
    }

    /// Starts the CLI loop
    pub fn run(&mut self) -> FileResult<()> {
        println!("🗂️  Welcome to the In-Memory File Management System!");
        println!("Type 'help' to see available commands.\n");

        loop {
            print!("file-cli> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .map_err(|e| FileError::InvalidInput(format!("Failed to read input: {}", e)))?;

            let operation = match Operation::from_str(&input) {
                Ok(op) => op,
                Err(e) => {
                    println!("❌ {}", e);
                    continue;
                }
            };

            match self.execute_operation(operation) {
                Ok(should_continue) => {
                    if !should_continue {
                        break;
                    }
                }
                Err(e) => println!("❌ {}", e),
            }
        }

        println!("👋 Goodbye!");
        Ok(())
    }

    /// Executes a CLI operation
    fn execute_operation(&mut self, operation: Operation) -> FileResult<bool> {
        match operation {
            Operation::Create => {
                self.create_file()?;
                Ok(true)
            }
            Operation::Write => {
                self.write_file()?;
                Ok(true)
            }
            Operation::Read => {
                self.read_file()?;
                Ok(true)
            }
            Operation::List => {
                self.list_files()?;
                Ok(true)
            }
            Operation::Delete => {
                self.delete_file()?;
                Ok(true)
            }
            Operation::Info => {
                self.show_file_info()?;
                Ok(true)
            }
            Operation::Help => {
                self.show_help()?;
                Ok(true)
            }
            Operation::Stats => {
                self.show_stats()?;
                Ok(true)
            }
            Operation::Quit => Ok(false),
        }
    }

    /// Creates a new file
    fn create_file(&mut self) -> FileResult<()> {
        println!("Creating file...");
        
        let name = self.get_input("Enter file name: ")?;
        let content = self.get_input("Enter file content: ")?;

        match self.filesystem.create_file(name.clone(), content) {
            Ok(id) => println!("✅ File '{}' created successfully with ID: {}", name, id),
            Err(e) => println!("❌ {}", e),
        }
        Ok(())
    }

    /// Writes content to an existing file
    fn write_file(&mut self) -> FileResult<()> {
        println!("Writing content...");
        
        let name = self.get_input("Enter file name: ")?;
        let content = self.get_input("Enter new content: ")?;

        match self.filesystem.write_file(&name, content) {
            Ok(()) => println!("✅ Content written to '{}' successfully", name),
            Err(e) => println!("❌ {}", e),
        }
        Ok(())
    }

    /// Reads a file's content
    fn read_file(&mut self) -> FileResult<()> {
        println!("Reading file...");
        
        let name = self.get_input("Enter file name: ")?;

        match self.filesystem.read_file(&name) {
            Ok(content) => {
                println!("📄 Content of '{}':", name);
                println!("{}", "-".repeat(40));
                println!("{}", content);
                println!("{}", "-".repeat(40));
            }
            Err(e) => println!("❌ {}", e),
        }
        Ok(())
    }

    /// Lists all files
    fn list_files(&mut self) -> FileResult<()> {
        println!("Listing files...");
        
        let files = self.filesystem.list_files();
        
        if files.is_empty() {
            println!("📭 No files found.");
        } else {
            println!("📂 Files in system:");
            for file in files {
                println!("  {}", file.display_summary());
            }
        }
        Ok(())
    }

    /// Deletes a file
    fn delete_file(&mut self) -> FileResult<()> {
        println!("Deleting file...");
        
        let input = self.get_input("Enter file name or ID: ")?;
        
        // Try to parse as ID first, then as name
        let result = if let Ok(id) = input.parse::<u32>() {
            self.filesystem.delete_file_by_id(id)
        } else {
            self.filesystem.delete_file(&input)
        };

        match result {
            Ok(()) => println!("✅ File deleted successfully"),
            Err(e) => println!("❌ {}", e),
        }
        Ok(())
    }

    /// Shows detailed file information
    fn show_file_info(&mut self) -> FileResult<()> {
        println!("File information...");
        
        let input = self.get_input("Enter file name or ID: ")?;
        
        // Try to parse as ID first, then as name
        let file = if let Ok(id) = input.parse::<u32>() {
            self.filesystem.get_file_by_id(id)
        } else {
            self.filesystem.get_file(&input)
        };

        match file {
            Ok(file) => {
                println!("📋 File Information:");
                println!("{}", file.display_detailed());
            }
            Err(e) => println!("❌ {}", e),
        }
        Ok(())
    }

    /// Shows help information
    fn show_help(&mut self) -> FileResult<()> {
        println!("📚 Available Commands:");
        println!("  create, c    - Create a new file");
        println!("  write, w     - Write content to an existing file");
        println!("  read, r      - Read file content");
        println!("  list, l, ls  - List all files");
        println!("  delete, d    - Delete a file (by name or ID)");
        println!("  info, i      - Show detailed file information");
        println!("  stats, s     - Show system statistics");
        println!("  help, h, ?   - Show this help message");
        println!("  quit, q      - Exit the program");
        Ok(())
    }

    /// Shows system statistics
    fn show_stats(&mut self) -> FileResult<()> {
        println!("📊 System Statistics:");
        println!("  Total files: {}", self.filesystem.file_count());
        println!("  Total size: {} bytes", self.filesystem.total_size());
        
        let files = self.filesystem.list_files();
        if !files.is_empty() {
            let avg_size = self.filesystem.total_size() / files.len();
            println!("  Average file size: {} bytes", avg_size);
            
            // Show file type distribution
            let mut extensions: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
            for file in files {
                let ext = file.extension().unwrap_or("no extension").to_string();
                *extensions.entry(ext).or_insert(0) += 1;
            }
            
            if !extensions.is_empty() {
                println!("  File types:");
                for (ext, count) in extensions {
                    println!("    .{}: {} files", ext, count);
                }
            }
        }
        Ok(())
    }

    /// Gets user input with a prompt
    fn get_input(&self, prompt: &str) -> FileResult<String> {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .map_err(|e| FileError::InvalidInput(format!("Failed to read input: {}", e)))?;

        let trimmed = input.trim().to_string();
        if trimmed.is_empty() {
            return Err(FileError::InvalidInput("Input cannot be empty".to_string()));
        }

        Ok(trimmed)
    }
}

impl Default for CLI {
    fn default() -> Self {
        Self::new()
    }
}
