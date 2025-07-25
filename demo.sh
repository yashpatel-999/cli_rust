#!/usr/bin/env bash

# File Management CLI - Demo Script
# This script demonstrates all features of the Rust CLI file management system

echo "🗂️  File Management CLI - Feature Demonstration"
echo "==============================================="
echo ""

echo "Starting the CLI application..."
echo "Commands to test all features:"
echo ""

echo "1. Show help:"
echo "   help"
echo ""

echo "2. Create files:"
echo "   create"
echo "   notes.txt"
echo "   This is my first note file with some content!"
echo ""
echo "   create"
echo "   todo.md"
echo "   # Todo List"
echo "   - Learn Rust ownership"
echo "   - Implement CLI features"
echo "   - Master error handling"
echo ""
echo "   create"
echo "   config.json"
echo "   {\"name\": \"File Manager\", \"version\": \"1.0\", \"debug\": true}"
echo ""

echo "3. List all files:"
echo "   list"
echo ""

echo "4. Read file content:"
echo "   read"
echo "   notes.txt"
echo ""

echo "5. Show detailed file info:"
echo "   info"
echo "   1"
echo ""

echo "6. Update file content:"
echo "   write"
echo "   notes.txt"
echo "   Updated note: Now with more detailed information about Rust!"
echo ""

echo "7. Show system statistics:"
echo "   stats"
echo ""

echo "8. Delete a file:"
echo "   delete"
echo "   config.json"
echo ""

echo "9. List files again (to see deletion):"
echo "   list"
echo ""

echo "10. Try to read deleted file (error demo):"
echo "    read"
echo "    config.json"
echo ""

echo "11. Show final stats:"
echo "    stats"
echo ""

echo "12. Exit:"
echo "    quit"
echo ""

echo "==============================================="
echo "Copy and paste these commands into the CLI one by one"
echo "to see all features in action!"
echo ""
echo "To start the application, run: cargo run"
