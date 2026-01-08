# sawit (saw)

A simple, lightweight CLI tool written in Rust that watches for file changes in a directory and executes a specified command.

## Features

- **Watch:** Recursively watches a directory or file for changes.
- **Execute:** specific command when a change is detected.
- **Cross-platform:** Works on Linux, macOS, and Windows.
- **Clear Screen:** Option to clear the terminal before running the command.

## Usage

```bash
saw --it <PATH> --do <COMMAND>
```

### Options

| Short | Long | Description | Default |
| :--- | :--- | :--- | :--- |
| `-i` | `--it` | Directory or file to watch | `.` (current dir) |
| `-d` | `--do` | Command to execute on change | (Required) |
| `-c` | `--clear` | Clear screen before executing | `false` |
| `-h` | `--help` | Print help information | |
| `-V` | `--version` | Print version information | |

### Examples

Watch the current directory and run `cargo check` on change:
```bash
saw -i . -d "cargo check"
```

Watch the `src` folder, clear the screen, and run tests:
```bash
saw --it src -c --do "cargo test"
```

Watch a specific file and run a python script:
```bash
saw -i script.py -d "python script.py"
```

## Installation

### From Source

```bash
git clone https://github.com/fepfitra/sawit.git
cd sawit
cargo install --path .
```

This will install the binary `sawit` and the shorthand `saw`.
