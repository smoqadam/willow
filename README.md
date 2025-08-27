# Willow

A smart file watcher and organizer that monitors directories for file changes and automatically performs actions based on configurable rules.

## What it does

Willow watches specified directories for file system events (created, modified, deleted) and applies rules to organize, move, rename, or log information about files. It features:

- **Smart stability detection**: Waits for files to finish downloading/copying before acting
- **Flexible conditions**: Match files by extension, glob patterns, regex, size, or content
- **Multiple actions**: Move, rename, or log file events
- **Template support**: Use dynamic placeholders in file paths and names
- **Temporary file handling**: Ignores browser download artifacts (.part, .crdownload, etc.)

## How to run

1. **Build the project:**
   ```bash
   cargo build --release
   ```

2. **Create a configuration file** (see example below)

3. **Run willow:**
   ```bash
   ./target/release/willow --config config.yaml
   ```

## Example Configuration

```yaml
watchers:
  - path: "/Users/username/Downloads"
    recursive: true
    ignore: ["part", "crdownload"]
    rules:
      # Organize images
      - conditions:
          - type: "extension"
            value: "jpg"
        actions:
          - type: "move"
            destination: "/Users/username/Pictures/"
      
      # Organize PDFs with logging
      - conditions:
          - type: "extension"
            value: "pdf"
        actions:
          - type: "move"
            destination: "/Users/username/Documents/PDFs/"
          - type: "log"
            message: "Moved PDF: {filename}"
      
      # Rename files with timestamp
      - conditions:
          - type: "glob"
            value: "screenshot*"
        actions:
          - type: "rename"
            template: "screenshot_{date}_{time}.{ext}"
```

### Configuration Options

- **path**: Directory to watch
- **recursive**: Watch subdirectories (true/false)
- **ignore**: File extensions to ignore as temporary files
- **conditions**: Rules for matching files:
  - `extension`: Match by file extension
  - `glob`: Match by glob pattern
  - `regex`: Match by regular expression
  - `size_gt`/`size_lt`: Match by file size
  - `contains`: Match by file content
- **actions**: What to do with matching files:
  - `move`: Move to destination directory
  - `rename`: Rename using template
  - `log`: Log a message

### Template Variables

Use these placeholders in move destinations and rename templates:

- `{filename}`: Full filename with extension
- `{name}`: Filename without extension
- `{ext}`: File extension
- `{date}`: Current date (YYYY-MM-DD)
- `{time}`: Current time (HH-MM-SS)
- `{datetime}`: Full timestamp

## Requirements

- Rust 1.70 or later
- macOS, Linux, or Windows
