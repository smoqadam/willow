# Willow

A smart file watcher and organizer that monitors directories for file changes and automatically performs actions based on configurable rules.

## What it does

Willow watches specified directories for file system events (created, modified, deleted) and applies rules to organize, move, or log information about files. It features:

- **Smart stability detection**: Waits for files to finish downloading/copying before acting
- **Flexible conditions**: Match files by extension, glob patterns, regex, size, or content
- **Multiple actions**: Move (supports renaming via templates) or log file events
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

### Dry-run mode

Preview actions without changing the filesystem:

```bash
./target/release/willow --config config.yaml --dry-run
```
Logs will include planned operations like `[dry-run] move src -> dest` and `[dry-run] create_dir_all path`.

## Example Configuration

```yaml
watchers:
  - path: "/Users/username/Downloads"
    recursive: true
    rules:
      - event: "created"
        conditions:
          - type: "extension"
            value: "jpg"
        actions:
          - type: "move"
            destination: "/Users/username/Pictures/"
      - event: "modified"
        conditions:
          - type: "extension"
            value: "pdf"
        actions:
          - type: "move"
            destination: "/Users/username/Documents/PDFs/"
```

This config moves `jpg` files to `"/Users/username/Pictures/"` when they are created, also moves `pdf`s to `Users/username/Documents/PDFs` on modify event.

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
  - `move`: Move to destination directory or file path template
    - optional `overwrite` policy: `error` (default), `skip`, `overwrite`, `suffix`
  - `log`: Log a message

### Template Variables

Use these placeholders in move destination templates:

- `{filename}`: Full filename with extension
- `{name}`: Filename without extension
- `{ext}`: File extension
- `{date}`: Current date (YYYY-MM-DD)
- `{time}`: Current time (HH-MM-SS)
- `{datetime}`: Full timestamp

### Overwrite Policy Examples

```yaml
actions:
  - type: "move"
    destination: "/Users/username/Pictures/"    # ends with `/` = directory
    overwrite: "suffix"                           # if exists, write file_1.ext, file_2.ext, ...

  - type: "move"
    destination: "{parent}/{name}_renamed.{ext}" # rename in-place using template
    overwrite: "overwrite"                        # replace destination if it exists
```

## Requirements

- Rust 1.70 or later
- macOS, Linux, or Windows
