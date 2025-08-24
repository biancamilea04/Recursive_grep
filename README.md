# Recursive Grep Tool

A command-line recursive grep utility written in Rust that searches for patterns in files within directories and subdirectories.

## Features

- **Recursive Search**: Search through all files in a directory and its subdirectories
- **Pattern Matching**: Support for both literal string matching and regular expressions
- **Case Insensitive Search**: Option to ignore case differences
- **Line Limiting**: Limit the maximum number of matching lines displayed
- **Count Only Mode**: Display only the count of matching lines
- **Interactive Shell**: Built-in interactive command-line interface

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Building from Source

1. Clone the repository:
```bash
git clone <repository-url>
cd Recursive_grep
```

2. Build the project:
```bash
cargo build --release
```

3. Run the executable:
```bash
cargo run
```

## Usage

The tool provides an interactive shell interface. After starting the program, you can use the following commands:

### Basic Syntax
```
grep [OPTIONS]... "PATTERN" "PATH/TO/DIRECTORY"
```

### Available Options

- `-m, --max_number_of_lines <NUMBER>`: Limit the maximum number of lines to display
- `-i, --ignore_case`: Ignore case differences between pattern and file content
- `-c, --only_count`: Display only the count of matching lines
- `-r, --regex_searching`: Enable regular expression pattern matching

### Examples

1. **Basic search**:
```
grep "hello" "/path/to/directory"
```

2. **Case-insensitive search**:
```
grep -i "Hello" "/path/to/directory"
```

3. **Limit results to 10 lines**:
```
grep -m 10 "pattern" "/path/to/directory"
```

4. **Count only matches**:
```
grep -c "pattern" "/path/to/directory"
```

5. **Regular expression search**:
```
grep -r "\d+" "/path/to/directory"
```

6. **Combined options**:
```
grep -i -m 5 -c "pattern" "/path/to/directory"
```

### Special Commands

- `grep --help`: Display help information
- `exit`: Exit the program

## Dependencies

- `walkdir` (2.5.0): For recursive directory traversal
- `regex` (1.11.1): For regular expression support

## Technical Details

- **Language**: Rust
- **Edition**: 2021
- **Binary Size**: Optimized for performance
- **Platform**: Cross-platform (Windows, macOS, Linux)

## Features in Detail

### Recursive Directory Traversal
The tool uses the `walkdir` crate to efficiently traverse directory structures, searching through all files in the specified path and its subdirectories.

### Pattern Matching
- **Literal matching**: Direct string comparison
- **Regular expressions**: Full regex support using the `regex` crate
- **Case sensitivity**: Optional case-insensitive matching

### Output Formatting
- Displays filename and matching lines
- Shows line numbers for context
- Supports ANSI escape sequence filtering
- Clean, readable output format

### Error Handling
- Graceful handling of file read errors
- Input validation for command arguments
- Clear error messages for invalid usage

## Development

### Project Structure
```
Recursive_grep/
├── src/
│   └── main.rs          # Main application code
├── Cargo.toml           # Project configuration
├── Cargo.lock           # Dependency lock file
└── README.md            # This file
```

### Building for Development
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Run `cargo fmt` and `cargo clippy`
6. Submit a pull request

## License

This project is open source. Please check the repository for license information.

## Performance

The tool is optimized for:
- Fast directory traversal
- Efficient pattern matching
- Memory-conscious file processing
- Quick startup time

## Troubleshooting

### Common Issues

1. **"Nu s-a putut obtine numele fisierului"**: File name encoding issue
2. **"Eroare la citire din fisier"**: File permission or access error
3. **"Argument invalid pentru max_number_of_lines"**: Non-numeric value provided for `-m` option

### Getting Help

Use the built-in help command:
```
grep --help
```

This will display detailed usage information and available options.
