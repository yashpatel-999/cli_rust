# File Management CLI - Rust Implementation

A comprehensive CLI-based in-memory File Management System built in Rust, demonstrating mastery of core and advanced Rust concepts.

## 🚀 Features

This project demonstrates the following Rust concepts:

### ✅ Core Rust Concepts Covered

1. **Variables & Mutability**
   - Mutable vector storage for file records
   - Mutable operations for file updates and deletions

2. **Data Types**
   - String types for file names and contents
   - Unsigned integers for file IDs and sizes
   - Boolean logic for operation control
   - Custom data types (File struct, Operation enum)

3. **Structs & Enums**
   - `File` struct with metadata (ID, name, content, size, timestamp)
   - `Operation` enum for CLI actions
   - `FileError` enum for error handling

4. **Pattern Matching**
   - Command parsing and dispatch
   - Error handling with match expressions
   - Operation execution control flow

5. **Ownership & Borrowing**
   - File creation takes ownership of data
   - File reading uses borrowed references
   - Mutable references for file updates

6. **References & Slices**
   - String slicing for content previews
   - Borrowed references in function signatures
   - Lifetime-aware string operations

7. **Functions**
   - Single-responsibility functions for each operation
   - Proper ownership models
   - Result-based error handling

8. **Traits & Generics**
   - Custom `FileDisplay` trait for consistent formatting
   - Generic error handling patterns
   - Trait implementations for File struct

9. **Lifetimes**
   - Lifetime annotations for borrowed data
   - Memory safety guarantees
   - Reference validation

10. **Modules & Crates**
    - Organized module structure (error, file, cli)
    - Proper visibility and encapsulation
    - Clean separation of concerns

11. **Error Handling**
    - Custom `FileError` enum
    - Result types throughout the codebase
    - Comprehensive error reporting

## 📁 Project Structure

```
src/
├── main.rs     # Entry point and CLI initialization
├── error.rs    # Custom error types and handling
├── file.rs     # File struct, FileSystem, and traits
└── cli.rs      # CLI interface and user interaction
```

## 🛠️ Building and Running

### Prerequisites
- Rust (latest stable version)
- Cargo package manager

### Build
```bash
cargo build
```

### Run
```bash
cargo run
```

## 🖥️ Usage

### Available Commands

| Command | Aliases | Description |
|---------|---------|-------------|
| `create` | `c` | Create a new file with content |
| `write` | `w` | Write new content to existing file |
| `read` | `r` | Read and display file content |
| `list` | `l`, `ls` | List all files with summary |
| `delete` | `d`, `del` | Delete file by name or ID |
| `info` | `i` | Show detailed file information |
| `stats` | `s` | Display system statistics |
| `help` | `h`, `?` | Show help information |
| `quit` | `q`, `exit` | Exit the program |

### Sample Usage Session

```
🗂️  Welcome to the In-Memory File Management System!
Type 'help' to see available commands.

file-cli> create
Creating file...
Enter file name: notes.txt
Enter file content: This is my first note in the system!
✅ File 'notes.txt' created successfully with ID: 1

file-cli> create
Creating file...
Enter file name: todo.md
Enter file content: # Todo List
- Learn Rust
- Build CLI app
- Master file systems
✅ File 'todo.md' created successfully with ID: 2

file-cli> list
Listing files...
📂 Files in system:
  [1] notes.txt (33 bytes)
  [2] todo.md (68 bytes)

file-cli> read
Reading file...
Enter file name: notes.txt
📄 Content of 'notes.txt':
----------------------------------------
This is my first note in the system!
----------------------------------------

file-cli> info
File information...
Enter file name or ID: 1
📋 File Information:
ID: 1
Name: notes.txt
Size: 33 bytes
Created: 5s ago
Preview: This is my first note in the system!

file-cli> stats
📊 System Statistics:
  Total files: 2
  Total size: 101 bytes
  Average file size: 50 bytes
  File types:
    .txt: 1 files
    .md: 1 files

file-cli> write
Writing content...
Enter file name: notes.txt
Enter new content: Updated note with more information!
✅ Content written to 'notes.txt' successfully

file-cli> delete
Deleting file...
Enter file name or ID: 2
✅ File deleted successfully

file-cli> quit
👋 Goodbye!
```

## 🎯 Key Features Demonstrated

### Memory Management
- Safe ownership transfers
- Borrowing for read operations
- Mutable borrowing for updates
- No memory leaks or dangling pointers

### Error Handling
- Comprehensive error types
- Result-based error propagation
- User-friendly error messages
- Graceful error recovery

### Type Safety
- Strong typing throughout
- Pattern matching for exhaustive handling
- Compile-time guarantees
- Zero-cost abstractions

### Performance
- In-memory storage for fast access
- Efficient string operations
- Minimal allocations
- O(1) operations where possible

### Code Organization
- Clean module separation
- Proper encapsulation
- Single responsibility principle
- Maintainable architecture

## 🔧 Advanced Features

### File Operations
- Unique ID generation
- Timestamp tracking
- Content preview (50 chars)
- File extension detection
- Size calculation

### CLI Interface
- Command aliases for efficiency
- Input validation
- Interactive prompts
- Colored output (emojis)
- Help system

### Statistics
- File count tracking
- Total size calculation
- Average file size
- File type distribution
- System overview

## 📚 Learning Outcomes

This project demonstrates proficiency in:

- **Rust Fundamentals**: Variables, types, functions, control flow
- **Ownership System**: Borrowing, lifetimes, memory safety
- **Error Handling**: Result types, custom errors, propagation
- **Traits & Generics**: Code reuse, type abstraction
- **Module System**: Code organization, visibility
- **CLI Development**: User interaction, command parsing
- **Data Structures**: Vectors, hashmaps, custom types
- **Pattern Matching**: Exhaustive handling, control flow

## 🎓 Rust Concepts Mastered

- ✅ Variables and Mutability
- ✅ Data Types (primitives and custom)
- ✅ Structs and Enums
- ✅ Pattern Matching
- ✅ Ownership and Borrowing
- ✅ References and Slices
- ✅ Functions and Methods
- ✅ Traits and Generics
- ✅ Lifetimes
- ✅ Modules and Crates
- ✅ Error Handling
- ✅ Collections and Iterators
- ✅ String handling
- ✅ I/O operations

## 🚧 Future Enhancements

Potential improvements could include:
- File searching and filtering
- File metadata (permissions, tags)
- Import/export functionality
- Configuration system
- Batch operations
- Plugin architecture

## 📄 License

This project is for educational purposes, demonstrating Rust programming concepts.
#   c l i _ r u s t  
 #   c l i _ r u s t  
 