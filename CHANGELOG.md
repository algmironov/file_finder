# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-02-08

### Added
- Initial release of FileFinder
- Parallel file system scanning with 8 threads
- Interactive drive selection
- File filtering by size and extension
- Duplicate file detection using SHA-256 hashing
- Real-time progress bars with spinners
- Colored terminal UI with file type icons
- Pagination for viewing large result sets
- File actions: open in explorer, delete
- Save and load scan results to/from JSON
- Cross-platform support (Windows, Linux, macOS)
- Comprehensive documentation
- Installation scripts for easy setup

### Features
- **Performance**: Scans millions of files in minutes
- **Usability**: Beautiful colored interface with emojis
- **Flexibility**: Multiple filtering options
- **Safety**: Confirmation prompts before deletion
- **Portability**: Single executable, no dependencies

### Technical
- Built with Rust 1.80+
- Uses jwalk for parallel directory traversal
- Uses rayon for parallel hash computation
- Uses dialoguer for interactive prompts
- Uses indicatif for progress bars

---

## [Unreleased]

### Planned Features
- Filter by file modification date
- Export results to CSV/Excel
- Group operations (bulk delete, move, etc.)
- Disk usage visualization
- Content-based search in text files
- Real-time file system monitoring
- GUI version (optional)

---

## Version History

### How to Read Version Numbers

Version format: MAJOR.MINOR.PATCH

- **MAJOR**: Breaking changes (incompatible API changes)
- **MINOR**: New features (backwards compatible)
- **PATCH**: Bug fixes (backwards compatible)

### Upgrade Notes

#### From 0.x to 1.0.0
- Initial stable release
- No breaking changes (first version)

---

## Contributing

Found a bug or want to contribute? 
- Report issues: https://github.com/yourusername/file_finder/issues
- Submit PRs: https://github.com/yourusername/file_finder/pulls

---

[1.0.0]: https://github.com/yourusername/file_finder/releases/tag/v1.0.0
[Unreleased]: https://github.com/yourusername/file_finder/compare/v1.0.0...HEAD
