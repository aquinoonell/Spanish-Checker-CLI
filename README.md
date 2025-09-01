# Spanish Checker

A command-line tool written in Rust for checking Spanish text files for grammar and spelling errors using the LanguageTool API.

## Features

- Check Spanish text files for errors
- Get correction suggestions
- View error context
- See error statistics and repeated mistakes

## Installation

1. Make sure you have Rust installed
2. Clone this repository
3. Run `cargo build --release`
4. The executable will be in `target/release/spanish-checker`

## Usage

```bash
spanish-checker examine <file.txt>
```

Example:
```bash
spanish-checker examine my_document.txt
```

## Output

For each error found, you'll see:
- Error category and description
- The incorrect text
- Up to 3 correction suggestions
- Context where the error appears

At the end, you get a summary with:
- Total number of errors
- Count of repeated mistakes

## Requirements

- Internet connection (uses LanguageTool's free API)
- Text files in Spanish

## Dependencies

- `clap` - Command line argument parsing
- `serde` - JSON serialization
- `tokio` - Async runtime
- `reqwest` - HTTP client


