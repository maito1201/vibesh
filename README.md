# vibesh

A safe shell wrapper for coding agents that blocks forbidden commands.

## Overview

`vibesh` is a Rust-based shell wrapper that reads a configuration file (`~/.vibeshrc`) to filter and block dangerous commands before passing allowed commands to the underlying shell (bash/zsh).

## Features

- 🛡️ **Command Filtering**: Block commands by exact match or regex pattern
- 🔄 **Shell Proxy**: Transparently proxy allowed commands to bash/zsh
- ⚡ **Fast & Lightweight**: Written in Rust for minimal overhead
- 📝 **Simple Configuration**: Easy-to-read `.vibeshrc` format

## Installation

### Prerequisites

- Rust toolchain (install from https://rustup.rs/)

### Build

```bash
cargo build --release
```

The binary will be available at `target/release/vibesh`.

## Configuration

Create a `~/.vibeshrc` file with the following format:

```
shell = bash
forbidden = rm
forbidden = /^sudo.*/
forbidden = regexp('git push --force')
```

### Configuration Options

| Key | Description | Example |
|-----|-------------|---------|
| `shell` | Backend shell (bash or zsh) | `shell = bash` |
| `forbidden` | Forbidden command (exact match) | `forbidden = rm` |
| `forbidden` | Forbidden pattern (regex) | `forbidden = /^sudo.*/` |
| `forbidden` | Forbidden pattern (function syntax) | `forbidden = regexp('pattern')` |

### Pattern Matching

**Exact Match:**
```
forbidden = rm
```
Blocks: `rm`, `rm file.txt`, `rm -rf /`

**Regex Match (slash syntax):**
```
forbidden = /^sudo.*/
```
Blocks: `sudo rm`, `sudo apt install`, etc.

**Regex Match (function syntax):**
```
forbidden = regexp('git.*--force')
```
Blocks: `git push --force`, `git commit --force`, etc.

## Usage

```bash
# Run vibesh
./target/release/vibesh

# Or with cargo
cargo run
```

### Example Session

```
vibesh shell (backend: bash)
Type 'exit' to quit
> ls
file1.txt  file2.txt

> rm test.txt
this command is not allowed

> echo "Hello, World!"
Hello, World!

> exit
```

## Testing

Run the test suite:

```bash
cargo test
```

## Architecture

The project is organized into four modules:

| Module | Responsibility | Lines |
|--------|----------------|-------|
| `config.rs` | Parse `.vibeshrc` configuration | ~133 |
| `filter.rs` | Command filtering logic | ~105 |
| `executor.rs` | Shell command execution | ~58 |
| `main.rs` | REPL loop | ~54 |

## How It Works

1. **Load Configuration**: Read and parse `~/.vibeshrc`
2. **User Input**: Receive command from user via REPL
3. **Extract Command**: Extract command name (ignore arguments)
4. **Filter**: Check against forbidden patterns
   - Exact match: Compare command name directly
   - Regex match: Test command against regex pattern
5. **Execute or Block**:
   - If allowed: Proxy to backend shell
   - If forbidden: Print "this command is not allowed"

## Security Considerations

- Only the **command name** is checked, arguments are ignored
- Example: `forbidden = rm` blocks `rm ./anything`
- Regex patterns can match more complex patterns
- Backend shell execution inherits stdio (output is not filtered)

## License

MIT
