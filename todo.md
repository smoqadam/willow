# Willow File Watcher - TODO

## Bugs

### Critical Bugs
- **CLI argument handling panic** - `main.rs:30` calls `unwrap()` on `cli.config` which will panic if no config file is provided, despite config being optional
- **Incomplete RenameAction implementation** - `actions/action.rs:64-69` RenameAction only logs but doesn't actually perform the rename operation
- **Unsafe array access in watcher** - `watcher/watcher.rs:42` uses direct indexing `[debounced_event.paths.len() - 1]` without checking if paths array is empty

### Minor Bugs
- **Error handling inconsistency** - Some functions use `unwrap()` while others properly return `Result<T>`. Example: `conditions.rs:26` uses `unwrap_or("")` but `conditions.rs:36` uses `unwrap_or("")` inconsistently
- **Invalid regex/glob patterns silently fail** - `conditions.rs:10-14` and `conditions.rs:17-21` return `false` for invalid patterns without logging the error

## Missing Features

### Core Features
- **TOML configuration support** - Currently only supports JSON config despite having a `config.toml` example file
- **Template variable substitution** - RenameAction and LogAction reference templates like `{name}_{date}.{ext}` and `{file}` but no substitution logic exists
- **Configuration validation** - No validation of config file structure, invalid configs will cause runtime panics
- **Signal handling** - No graceful shutdown on SIGINT/SIGTERM, watcher runs indefinitely
- **Multiple watcher paths per rule** - Each watcher can only watch one path, can't watch multiple directories with same rules

### Action Features
- **Copy action** - Only Move action exists, no way to copy files while keeping originals
- **Execute/Command action** - No way to run shell commands or scripts when files match conditions
- **Backup action** - No built-in way to create backups before moving/renaming files
- **Delete action** - No way to automatically delete files that match conditions

### Condition Features
- **File age conditions** - No way to match files based on creation/modification time (e.g., files older than X days)
- **MIME type detection** - Only extension-based file type checking, no actual content-based MIME detection
- **Nested directory conditions** - No way to match based on directory depth or parent directory names
- **File permissions conditions** - No way to match based on file permissions or ownership

## Edge Cases

### File System Edge Cases
- **Rapid file modifications** - Debouncer only processes last event, intermediate modifications might be lost
- **Network mounted filesystems** - No handling for network filesystem-specific events or permissions
- **Symlink handling** - No explicit handling of symbolic links, unclear behavior when symlinks are created/modified
- **Cross-filesystem moves** - Move action might fail when moving across filesystem boundaries (should fall back to copy+delete)
- **Large file operations** - No progress indication or chunked processing for large file moves/copies

### Configuration Edge Cases
- **Circular move operations** - No prevention of moving files to directories that would create infinite loops
- **Conflicting rules** - Multiple rules could match same file, no priority system or conflict resolution
- **Invalid destination paths** - Move/Rename actions don't validate that destination directories exist or are writable
- **Path traversal vulnerabilities** - No validation of destination paths to prevent writing outside intended directories

### Runtime Edge Cases
- **Resource exhaustion** - No limits on number of watched directories or maximum file queue size
- **Permission errors** - Limited error handling when file operations fail due to insufficient permissions
- **Concurrent access** - No handling of files being modified by other processes during operations

## Nice to Have

### User Experience
- **Dry run mode** - Preview what actions would be taken without actually performing them
- **Interactive mode** - Prompt user for confirmation before destructive operations
- **Progress bars/status** - Visual feedback for long-running operations like moving large files
- **Configuration wizard** - Interactive setup to generate initial config files
- **Hot config reload** - Ability to reload configuration without restarting the application

### Monitoring & Debugging
- **Metrics/statistics** - Track number of files processed, actions executed, errors encountered
- **Web dashboard** - Browser-based interface to monitor file watcher status and statistics
- **Rule testing utility** - Tool to test if specific files would match given rules without running watcher
- **Detailed logging levels** - More granular logging control (trace, debug, info, warn, error)
- **Log rotation** - Automatic log file rotation to prevent disk space issues

### Advanced Features
- **Plugin system** - Allow custom actions and conditions through external plugins or scripts
- **Conditional chaining** - Support for complex condition logic (AND, OR, NOT operations)
- **Scheduled actions** - Time-based rules that don't depend on file system events
- **File content analysis** - Actions based on file content analysis (text parsing, metadata extraction)
- **Integration APIs** - REST API or webhook support for external system integration

### Performance & Reliability
- **Parallel processing** - Process multiple file operations concurrently
- **Retry mechanisms** - Automatic retry for failed operations with exponential backoff
- **Checksum verification** - Verify file integrity after move/copy operations
- **Transaction support** - Atomic operations that can be rolled back if any step fails
- **Memory optimization** - More efficient handling of large numbers of watched files

### Configuration & Deployment
- **Environment variable substitution** - Support for environment variables in config files
- **Multiple config files** - Ability to load and merge multiple configuration files
- **Config file validation** - Schema validation with helpful error messages
- **Docker support** - Official Docker images and container-optimized builds
- **Systemd service files** - Official service definitions for Linux distributions