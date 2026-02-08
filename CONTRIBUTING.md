# Contributing to FileFinder

First off, thank you for considering contributing to FileFinder! 🎉

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Coding Guidelines](#coding-guidelines)
- [Testing](#testing)

## 📜 Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). 
Please be respectful and constructive in all interactions.

## 🤝 How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates.

**When submitting a bug report, include:**
- Clear, descriptive title
- Steps to reproduce
- Expected behavior
- Actual behavior
- Your environment (OS, Rust version, etc.)
- Error messages or screenshots

**Example:**
```markdown
**Bug**: Scan hangs on network drives

**Steps to reproduce:**
1. Run `file_finder -p Z:\`
2. Program becomes unresponsive

**Environment:**
- OS: Windows 11
- Rust: 1.80.0
- FileFinder: 1.0.0

**Expected:** Scan completes or shows error
**Actual:** Program hangs indefinitely
```

### Suggesting Features

Feature requests are welcome! Please include:
- Clear description of the feature
- Why it would be useful
- Possible implementation approach
- Examples of similar features in other tools

### Pull Requests

We actively welcome your pull requests!

**Good first issues** are labeled `good-first-issue` - perfect for newcomers!

## 🛠️ Development Setup

### Prerequisites

1. **Rust 1.80+**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Git**
   ```bash
   git --version
   ```

3. **Windows:** Visual Studio Build Tools
   - Download: https://aka.ms/vs/17/release/vs_buildtools.exe
   - Install "Desktop development with C++"

### Fork and Clone

```bash
# Fork on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/file_finder.git
cd file_finder

# Add upstream remote
git remote add upstream https://github.com/ORIGINAL_OWNER/file_finder.git
```

### Build

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run
cargo run -- --help
```

### Run Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_scan_basic

# With output
cargo test -- --nocapture
```

## 🔄 Pull Request Process

### 1. Create a Branch

```bash
git checkout -b feature/my-awesome-feature
# or
git checkout -b fix/some-bug
```

Branch naming:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation
- `refactor/` - Code refactoring
- `test/` - Adding tests

### 2. Make Your Changes

- Write clear, concise code
- Follow the coding guidelines below
- Add tests for new features
- Update documentation if needed

### 3. Commit

```bash
git add .
git commit -m "Add feature: description of what you did"
```

**Commit message format:**
```
<type>: <subject>

<body (optional)>

<footer (optional)>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting, missing semicolons, etc.
- `refactor`: Code restructuring
- `test`: Adding tests
- `chore`: Updating build tasks, etc.

**Examples:**
```
feat: add filter by modification date

- Add --date-from and --date-to options
- Filter files by modification date range
- Update documentation

fix: correct banner alignment

The right border was misaligned due to incorrect character count.
Fixed by adding one more '═' character.
```

### 4. Push and Create PR

```bash
git push origin feature/my-awesome-feature
```

Then go to GitHub and create a Pull Request:
- Fill in the PR template
- Link related issues
- Request review if needed

### 5. Code Review

- Address review comments
- Push additional commits if needed
- Be responsive and collaborative

### 6. Merge

Once approved, a maintainer will merge your PR. Thank you! 🎉

## 📝 Coding Guidelines

### Rust Style

Follow the official [Rust Style Guide](https://doc.rust-lang.org/beta/style-guide/):

```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy
```

### Code Organization

```rust
// Good: Clear, documented function
/// Scans files matching the given configuration
///
/// # Arguments
/// * `config` - Scan configuration with paths and filters
///
/// # Returns
/// * `Result<ScanResults>` - Scan results or error
pub fn scan_files(config: ScanConfig) -> Result<ScanResults> {
    // Implementation
}

// Bad: Unclear, undocumented
pub fn scan(c: SC) -> Res {
    // What does this do?
}
```

### Error Handling

```rust
// Good: Descriptive error messages
let file = File::open(path)
    .with_context(|| format!("Failed to open file: {:?}", path))?;

// Bad: Generic error
let file = File::open(path)?;
```

### Comments

- Write comments for complex logic
- Use doc comments (`///`) for public API
- Keep comments up-to-date with code

```rust
// Good
/// Calculates SHA-256 hash of a file
/// Reads the file in 8KB chunks for memory efficiency
pub fn calculate_file_hash<P: AsRef<Path>>(path: P) -> Result<String> {
    // ...
}

// Bad
// hash function
pub fn calculate_file_hash<P: AsRef<Path>>(path: P) -> Result<String> {
    // ...
}
```

### Naming Conventions

- `snake_case` for functions and variables
- `PascalCase` for types and traits
- `SCREAMING_SNAKE_CASE` for constants
- Clear, descriptive names

```rust
// Good
const MAX_BUFFER_SIZE: usize = 8192;
fn calculate_total_size(files: &[FileInfo]) -> u64 { }

// Bad
const MAXBUF: usize = 8192;
fn calc(f: &[FileInfo]) -> u64 { }
```

## 🧪 Testing

### Write Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size_string() {
        assert_eq!(parse_size_string("100MB").unwrap(), 100 * 1024 * 1024);
        assert_eq!(parse_size_string("1GB").unwrap(), 1024 * 1024 * 1024);
    }

    #[test]
    fn test_scan_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let config = ScanConfig::new(vec![temp_dir.path().to_string()], 0);
        let results = scan_files(config).unwrap();
        assert_eq!(results.files.len(), 0);
    }
}
```

### Test Coverage

Aim for:
- All public functions have tests
- Edge cases are covered
- Error paths are tested

## 📚 Documentation

### Code Documentation

```rust
/// Short description
///
/// Longer description with details about behavior,
/// edge cases, and usage examples.
///
/// # Arguments
/// * `param1` - Description of parameter 1
/// * `param2` - Description of parameter 2
///
/// # Returns
/// Description of return value
///
/// # Errors
/// When and why this function might return an error
///
/// # Examples
/// ```
/// let result = function(arg1, arg2);
/// assert_eq!(result, expected);
/// ```
pub fn function(param1: Type1, param2: Type2) -> Result<ReturnType> {
    // ...
}
```

### Update README

If your change affects usage:
- Update README.md
- Update EXAMPLES.md
- Add to CHANGELOG.md

## ❓ Questions?

- Open an issue for clarification
- Join discussions
- Check existing documentation

## 🙏 Thank You!

Your contributions make FileFinder better for everyone! 

Every contribution counts:
- 🐛 Bug reports
- 💡 Feature ideas  
- 📝 Documentation improvements
- 🔧 Code contributions
- ⭐ Stars and shares

Thank you for being awesome! 🌟
