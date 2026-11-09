# FileFinder 🔍

**An interactive Rust CLI for finding large files, scanning directories, and detecting duplicates on Windows.**

[**Русская документация / Original Russian README**](README.ru.md)

FileFinder helps inspect local disk usage, filter large files, and identify duplicate content. It combines a command-line interface with interactive navigation and uses Rust libraries such as `jwalk`, `rayon`, and `sha2`.

> **Project status:** A personal utility that is useful in practice but still needs refinement. Review results carefully before deleting any files.

## Features

- Scan selected disks or directories with parallel traversal.
- Filter files by minimum size and extension.
- Identify duplicates using SHA-256 hashes.
- Browse paginated results in an interactive terminal.
- Open file locations, inspect details, and optionally delete selected files.
- Save scan results to JSON and load them later.
- Show progress and human-readable file sizes.

Performance varies with the filesystem, storage hardware, file count, permissions, and whether hashing is enabled. No reproducible benchmarks are currently provided.

## Requirements

- Windows (the documented and intended platform).
- A Rust toolchain to build from source: [rustup.rs](https://rustup.rs/).

## Build

```bash
git clone https://github.com/algmironov/file_finder.git
cd file_finder
cargo build --release
```

The Windows executable is produced at `target/release/file_finder.exe`.

You can also install the locally built CLI into your Cargo binary directory:

```bash
cargo install --path .
```

## Quick start

```powershell
# Interactive disk selection
file_finder.exe

# Scan a specific disk
file_finder.exe -p C:\

# Find files larger than 500 MB
file_finder.exe --min-size 500MB

# Filter by file extension
file_finder.exe -e mp4,mkv,avi

# Find duplicates
file_finder.exe --duplicates

# Save results
file_finder.exe --output results.json
```

If the executable is not on your `PATH`, run it using its full path or from its build directory.

## CLI options

| Option | Short | Description |
|---|---|---|
| `--min-size` | `-m` | Minimum file size, such as `100MB` or `1GB` |
| `--extensions` | `-e` | Comma-separated extension filter |
| `--paths` | `-p` | One or more scan paths |
| `--output` | `-o` | Save results as JSON |
| `--load` | `-l` | Load previously saved results |
| `--duplicates` | `-d` | Search for duplicate content |
| `--page-size` | | Number of entries per page |
| `--no-interactive` | | Scan without the interactive menu |

For example:

```powershell
# Scan two disks and find large video files
file_finder.exe -p C:\ -p D:\ --min-size 1GB -e mp4,mkv,avi

# Export document scan results without entering the interactive menu
file_finder.exe --min-size 50MB -e pdf,docx,xlsx --no-interactive --output documents.json

# Load saved results
file_finder.exe --load documents.json
```

## Interactive mode

After scanning, you can navigate through results, inspect file details, apply extension filters, open a file's location in Explorer, and perform supported file actions.

**Deletion warning:** Removing a file is a destructive operation. Confirm that a file is no longer needed and that any detected duplicate is genuinely safe to remove. Do not treat a matching hash as permission to delete automatically.

## Known limitations

- Some system files may be inaccessible without appropriate permissions.
- Network drives can be substantially slower than local disks.
- Hashing large files can take considerable time and disk I/O.
- The tool is not yet presented as a fully tested or polished general-purpose file manager.

## Future improvements

Potential improvements include more robust file operations, better diagnostics, improved distribution, and optional reporting features. These are ideas, not promises or release commitments.

## License

MIT. See the repository license file for the applicable terms.

---

For the original, more detailed instructions and historical performance notes, see [README.ru.md](README.ru.md).
