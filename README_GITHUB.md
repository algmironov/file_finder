# 🔍 FileFinder

> Fast and efficient CLI tool for finding large files on Windows with interactive features

[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](https://github.com/algmironov/file_finder)

**[English](#english)** | **[Русский](#russian)**

---

<a name="english"></a>
## 🌟 Features

- ⚡ **Blazing Fast** - Parallel scanning with 8 threads
- 🎯 **Interactive** - Select drives, browse results with pagination
- 🔍 **Find Duplicates** - SHA-256 based duplicate detection
- 📊 **Real-time Progress** - Live progress bars and statistics
- 🎨 **Beautiful UI** - Colored terminal interface with file icons
- 💾 **Save/Load Results** - Export and import scan results as JSON
- 🔧 **Powerful Filters** - Filter by size, file type, and more

## 📸 Screenshots

```
╔════════════════════════════════════════╗
║      🔍 FileFinder v1.0 🔍            ║
║    Fast file finder utility            ║
╚════════════════════════════════════════╝

Minimum file size: 500.0 MB
Scanning paths: ["C:\\"]

⠋ [=================================>] 1.2M files scanned
✓ Scan complete! Scanned: 1234567 | Found: 145

=== Scan Statistics ===
⏱️  Scan time: 48 sec
📊 Total scanned: 1234567 files
✅ Found matching: 145 files
💾 Total size: 127.3 GB
📈 Largest file: huge_movie.mkv (8.5 GB)
```

## 🚀 Quick Start

### Prerequisites

- [Rust 1.80+](https://rustup.rs/)
- Windows: [Visual Studio Build Tools](https://aka.ms/vs/17/release/vs_buildtools.exe) with "Desktop development with C++"

### Installation

#### Option 1: From Source

```bash
git clone https://github.com/algmironov/file_finder.git
cd file_finder
cargo build --release
```

The executable will be in `target/release/file_finder.exe`

#### Option 2: Install with Cargo

```bash
cargo install --git https://github.com/algmironov/file_finder.git
```

#### Option 3: Download Binary

Download the latest release from the [Releases](https://github.com/algmironov/file_finder/releases) page.

### Usage

```bash
# Interactive mode (select drives)
file_finder

# Scan specific drive
file_finder -p C:\

# Find files larger than 500MB
file_finder --min-size 500MB

# Filter by file types
file_finder -e mp4,mkv,avi --min-size 200MB

# Find duplicates
file_finder --min-size 100MB --duplicates

# Save results
file_finder --min-size 1GB -o results.json
```

## 📖 Documentation

- **[Installation Guide](docs/INSTALLATION.md)** - Detailed setup instructions
- **[User Guide](docs/USER_GUIDE.md)** - How to use all features
- **[Examples](EXAMPLES.md)** - 50+ usage examples
- **[Contributing](CONTRIBUTING.md)** - How to contribute

## 💡 Use Cases

- 🧹 **Disk Cleanup** - Find and remove large unnecessary files
- 🔄 **Duplicate Detection** - Find and delete duplicate files
- 📊 **Disk Analysis** - Understand what's taking up space
- 🎬 **Media Management** - Organize video/audio collections
- 📦 **Archive Cleanup** - Find old backups and archives

## 🛠️ Command Line Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--min-size` | `-m` | Minimum file size (e.g., 100MB, 1GB) | 100MB |
| `--extensions` | `-e` | Filter by extensions (comma-separated) | - |
| `--paths` | `-p` | Paths to scan | Interactive |
| `--output` | `-o` | Save results to JSON | - |
| `--load` | `-l` | Load results from JSON | - |
| `--duplicates` | `-d` | Find duplicate files | false |
| `--page-size` | | Files per page | 20 |
| `--no-interactive` | | Skip interactive mode | false |

## 🏗️ Architecture

```
file_finder/
├── src/
│   ├── main.rs          # CLI interface
│   ├── scanner/         # Parallel file scanning
│   ├── ui/              # Interactive menus
│   ├── models/          # Data structures
│   └── utils/           # Helper functions
├── tests/               # Integration tests
└── Cargo.toml          # Dependencies
```

## 🔧 Building from Source

### Windows

```bash
# Install Rust
# Download from https://rustup.rs/

# Install Visual Studio Build Tools
# Download from https://aka.ms/vs/17/release/vs_buildtools.exe
# Select "Desktop development with C++"

# Build
git clone https://github.com/algmironov/file_finder.git
cd file_finder
cargo build --release
```

### Linux/macOS

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
git clone https://github.com/algmironov/file_finder.git
cd file_finder
cargo build --release
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_scan_basic
```

## 📦 Dependencies

- **clap** - Command line argument parsing
- **jwalk** - Parallel directory traversal
- **dialoguer** - Interactive prompts
- **indicatif** - Progress bars
- **serde** - Serialization
- **rayon** - Data parallelism
- **sha2** - File hashing

See [Cargo.toml](Cargo.toml) for complete list.

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md).

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [jwalk](https://github.com/Byron/jwalk) for fast parallel scanning
- Inspired by tools like WinDirStat and TreeSize

## 📞 Support

- 🐛 [Report a Bug](https://github.com/algmironov/file_finder/issues)
- 💡 [Request a Feature](https://github.com/algmironov/file_finder/issues)
- 📖 [Documentation](https://github.com/algmironov/file_finder/wiki)

## 🌟 Star History

If you find this project useful, please consider giving it a star! ⭐

---

<a name="russian"></a>
## 🌟 Особенности

- ⚡ **Быстрая работа** - Параллельное сканирование в 8 потоков
- 🎯 **Интерактивность** - Выбор дисков, пагинация результатов
- 🔍 **Поиск дубликатов** - Определение по SHA-256 хешу
- 📊 **Прогресс в реальном времени** - Индикаторы и статистика
- 🎨 **Красивый интерфейс** - Цветной вывод с иконками файлов
- 💾 **Сохранение результатов** - Экспорт и импорт в JSON
- 🔧 **Мощные фильтры** - По размеру, типу файлов и др.

## 🚀 Быстрый старт

### Требования

- [Rust 1.80+](https://rustup.rs/)
- Windows: [Visual Studio Build Tools](https://aka.ms/vs/17/release/vs_buildtools.exe) с компонентом "Desktop development with C++"

### Установка

#### Вариант 1: Из исходников

```bash
git clone https://github.com/algmironov/file_finder.git
cd file_finder
cargo build --release
```

Исполняемый файл будет в `target/release/file_finder.exe`

#### Вариант 2: Через Cargo

```bash
cargo install --git https://github.com/algmironov/file_finder.git
```

#### Вариант 3: Скачать готовый файл

Скачайте последнюю версию со страницы [Releases](https://github.com/algmironov/file_finder/releases).

### Использование

```bash
# Интерактивный режим (выбор дисков)
file_finder

# Сканирование конкретного диска
file_finder -p C:\

# Поиск файлов больше 500MB
file_finder --min-size 500MB

# Фильтр по типам файлов
file_finder -e mp4,mkv,avi --min-size 200MB

# Поиск дубликатов
file_finder --min-size 100MB --duplicates

# Сохранение результатов
file_finder --min-size 1GB -o results.json
```

## 📖 Документация

- **[Руководство по установке](docs/INSTALLATION_RU.md)** - Подробная инструкция
- **[Руководство пользователя](docs/USER_GUIDE_RU.md)** - Все возможности
- **[Примеры](EXAMPLES.md)** - 50+ примеров использования
- **[Участие в разработке](CONTRIBUTING_RU.md)** - Как помочь проекту

## 💡 Применение

- 🧹 **Очистка диска** - Поиск и удаление больших файлов
- 🔄 **Поиск дубликатов** - Нахождение и удаление копий
- 📊 **Анализ диска** - Понимание использования места
- 🎬 **Управление медиа** - Организация видео/аудио
- 📦 **Очистка архивов** - Поиск старых резервных копий

## 🤝 Участие в разработке

Мы приветствуем ваш вклад! Прочитайте [Руководство по участию](CONTRIBUTING_RU.md).

1. Форкните репозиторий
2. Создайте ветку (`git checkout -b feature/amazing-feature`)
3. Закоммитьте изменения (`git commit -m 'Add amazing feature'`)
4. Запушьте в ветку (`git push origin feature/amazing-feature`)
5. Откройте Pull Request

## 📝 Лицензия

Проект распространяется под лицензией MIT - см. файл [LICENSE](LICENSE).

## 📞 Поддержка

- 🐛 [Сообщить об ошибке](https://github.com/algmironov/file_finder/issues)
- 💡 [Предложить функцию](https://github.com/algmironov/file_finder/issues)
- 📖 [Документация](https://github.com/algmironov/file_finder/wiki)

---

Made with ❤️ and Rust
