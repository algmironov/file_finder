# Структура проекта FileFinder

```
file_finder/
│
├── Cargo.toml                 # Конфигурация проекта и зависимости
├── Cargo.lock                 # Точные версии зависимостей (генерируется автоматически)
│
├── src/                       # Исходный код
│   ├── main.rs               # Точка входа, CLI интерфейс
│   ├── lib.rs                # Экспорт модулей для библиотеки/тестов
│   │
│   ├── models/               # Модели данных
│   │   └── mod.rs           # FileInfo, ScanResults, FileAction
│   │
│   ├── scanner/              # Сканирование файловой системы
│   │   └── mod.rs           # Параллельное сканирование, поиск дубликатов
│   │
│   ├── ui/                   # Пользовательский интерфейс
│   │   └── mod.rs           # Интерактивные меню, пагинация, отображение
│   │
│   └── utils/                # Вспомогательные функции
│       └── mod.rs           # Получение дисков, хеширование, иконки
│
├── tests/                     # Интеграционные тесты
│   └── integration_tests.rs  # Тесты сканирования и утилит
│
├── target/                    # Скомпилированные файлы (создается cargo)
│   ├── debug/                # Debug сборка
│   └── release/              # Release сборка (оптимизированная)
│       └── file_finder.exe   # Исполняемый файл
│
├── build.bat                  # Скрипт сборки для Windows
├── build.sh                   # Скрипт сборки для Linux/Mac
│
├── README.md                  # Основная документация
├── EXAMPLES.md               # Примеры использования
├── IMPROVEMENTS.md           # Идеи для будущих улучшений
├── DISTRIBUTION.md           # Руководство по распространению
├── RUST_GUIDE.md             # Шпаргалка по Rust
│
├── .gitignore                # Игнорируемые Git файлы
└── LICENSE                   # Лицензия (нужно добавить)
```

## Описание модулей

### src/main.rs
**Назначение**: Точка входа в программу
**Содержит**:
- Определение CLI аргументов с помощью clap
- Логика запуска программы
- Обработка команд пользователя
- Вывод статистики и результатов

**Основные функции**:
```rust
fn main() -> Result<()>                    // Точка входа
fn print_banner()                          // Приветствие
fn print_statistics(results: &ScanResults) // Статистика
fn save_results(...)                       // Сохранение в JSON
fn load_and_display_results(...)          // Загрузка из JSON
```

### src/lib.rs
**Назначение**: Экспорт модулей для использования в библиотеке
**Содержит**:
- Публичные экспорты всех модулей

### src/models/mod.rs
**Назначение**: Структуры данных
**Содержит**:
```rust
pub struct FileInfo {
    path: PathBuf,
    size: u64,
    extension: String,
    modified: DateTime<Local>,
    hash: Option<String>,
}

pub struct ScanResults {
    scan_start: DateTime<Local>,
    scan_end: DateTime<Local>,
    scanned_paths: Vec<String>,
    min_size_bytes: u64,
    extension_filter: Option<Vec<String>>,
    files: Vec<FileInfo>,
    total_scanned: u64,
    total_size: u64,
}

pub enum FileAction {
    OpenLocation,
    Delete,
    ShowDetails,
    Cancel,
}
```

### src/scanner/mod.rs
**Назначение**: Ядро приложения - сканирование файловой системы
**Содержит**:
```rust
pub struct ScanConfig {
    paths: Vec<String>,
    min_size: u64,
    extensions: Option<Vec<String>>,
    show_progress: bool,
}

pub fn scan_files(config: ScanConfig) -> Result<ScanResults>
    // Параллельное сканирование с jwalk
    // Прогресс-бар
    // Фильтрация по размеру и расширениям
    
pub fn find_duplicates(files: &mut [FileInfo]) -> Result<Vec<Vec<FileInfo>>>
    // Параллельное хеширование
    // Группировка по хешу
```

**Ключевые особенности**:
- Использует `jwalk` для параллельного обхода (8 потоков)
- Атомарные счетчики для многопоточного доступа
- Прогресс-бар с `indicatif`
- Параллельное хеширование с `rayon`

### src/ui/mod.rs
**Назначение**: Интерактивный пользовательский интерфейс
**Содержит**:
```rust
pub fn select_drives(available_drives: Vec<String>) -> Result<Vec<String>>
    // Множественный выбор дисков
    
pub fn display_files_paginated(...) -> Result<()>
    // Таблица с пагинацией
    // Навигация по страницам
    // Выбор файлов
    
fn file_action_menu(file: &FileInfo) -> Result<()>
    // Меню действий с файлом
    // Открыть, удалить, показать детали
    
pub fn display_duplicates(duplicate_groups: Vec<Vec<FileInfo>>) -> Result<()>
    // Отображение групп дубликатов
    // Предложение удаления
```

**Используемые библиотеки**:
- `dialoguer` - интерактивные меню
- `colored` - цветной вывод

### src/utils/mod.rs
**Назначение**: Вспомогательные утилиты
**Содержит**:
```rust
pub fn get_available_drives() -> Result<Vec<String>>
    // Получение списка дисков (Windows API)
    
pub fn format_size(bytes: u64) -> String
    // Форматирование: 1024 -> "1.0 KB"
    
pub fn calculate_file_hash<P: AsRef<Path>>(path: P) -> Result<String>
    // SHA-256 хеширование файла
    
pub fn open_file_location<P: AsRef<Path>>(path: P) -> Result<()>
    // Открыть проводник
    
pub fn delete_file<P: AsRef<Path>>(path: P) -> Result<()>
    // Удалить файл
    
pub fn get_file_icon(extension: &str) -> &str
    // Иконка эмодзи для типа файла
    
pub fn parse_size_string(size_str: &str) -> Result<u64>
    // Парсинг "100MB" -> 104857600
```

### tests/integration_tests.rs
**Назначение**: Автоматические тесты
**Содержит**:
- Тесты сканирования с разными конфигурациями
- Тесты фильтрации по размеру и расширению
- Тесты утилит (форматирование, парсинг, иконки)

## Зависимости (Cargo.toml)

### Основные
- **clap** - Парсинг аргументов командной строки
- **jwalk** - Параллельный обход директорий
- **dialoguer** - Интерактивные меню
- **indicatif** - Прогресс-бары
- **serde/serde_json** - Сериализация в JSON
- **rayon** - Параллелизм данных
- **sha2** - Хеширование файлов

### Вспомогательные
- **chrono** - Работа с датами
- **bytesize** - Форматирование размеров
- **colored** - Цветной текст
- **anyhow/thiserror** - Обработка ошибок
- **opener** - Открытие файлов/папок
- **windows** - Windows API (только на Windows)

### Для разработки
- **tempfile** - Временные файлы для тестов

## Файлы документации

### README.md
- Описание проекта
- Инструкции по установке
- Базовое использование
- Аргументы командной строки
- Примеры

### EXAMPLES.md
- Множество готовых примеров команд
- Сценарии использования
- Комбинации фильтров
- Советы и трюки

### IMPROVEMENTS.md
- Реализованные функции
- Идеи для будущих улучшений
- Приоритизация
- Как внести вклад

### DISTRIBUTION.md
- Способы распространения
- Создание installers
- Публикация на GitHub
- Автообновление
- Code signing

### RUST_GUIDE.md
- Основы Rust для новичков
- Объяснение конструкций кода
- Типичные ошибки
- Полезные ресурсы

## Процесс сборки

### Debug сборка (для разработки)
```bash
cargo build
# Создает: target/debug/file_finder.exe
# Размер: ~15-20 MB
# Быстрая компиляция, медленное выполнение
```

### Release сборка (для распространения)
```bash
cargo build --release
# Создает: target/release/file_finder.exe
# Размер: ~6-8 MB (после оптимизаций)
# Медленная компиляция, быстрое выполнение
# + LTO, strip, оптимизации
```

### Оптимизации в release (Cargo.toml)
```toml
[profile.release]
opt-level = 3        # Максимальная оптимизация
lto = true           # Link Time Optimization
codegen-units = 1    # Лучшая оптимизация (медленнее компиляция)
strip = true         # Удаление debug символов
```

## Workflow разработки

1. **Редактирование кода**
   ```bash
   # Быстрая проверка без сборки
   cargo check
   ```

2. **Тестирование**
   ```bash
   cargo test
   ```

3. **Запуск**
   ```bash
   cargo run -- --min-size 100MB
   ```

4. **Форматирование**
   ```bash
   cargo fmt
   ```

5. **Линтинг**
   ```bash
   cargo clippy
   ```

6. **Сборка release**
   ```bash
   cargo build --release
   ```

## Размеры скомпилированных файлов

| Версия | Размер | Примечание |
|--------|--------|------------|
| Debug  | ~15-20 MB | С debug символами |
| Release (обычная) | ~8-10 MB | Оптимизации |
| Release (strip) | ~6-8 MB | Удалены debug символы |
| Release + UPX | ~2-3 MB | Сжатие |

## Производительность

### Сканирование
- **500k файлов**: ~30-60 секунд (SSD)
- **1M файлов**: ~1-2 минуты (SSD)
- **5M файлов**: ~5-10 минут (SSD)

### Хеширование (поиск дубликатов)
- **Зависит от размера файлов**
- Параллельно на всех ядрах CPU
- ~100 MB/s на файл (зависит от диска)

### Память
- Базовое использование: ~10-50 MB
- При сканировании: +5-10 MB на каждые 100k файлов
- При хешировании: +buffer (8KB на файл в данный момент)

## Расширение проекта

### Добавление нового модуля
1. Создайте `src/new_module/mod.rs`
2. Добавьте в `src/lib.rs`: `pub mod new_module;`
3. Используйте: `use crate::new_module::...;`

### Добавление новой зависимости
1. Найдите на https://crates.io/
2. Добавьте в `Cargo.toml`:
   ```toml
   [dependencies]
   new_crate = "1.0"
   ```
3. Используйте: `use new_crate::...;`

### Добавление тестов
1. Unit тесты - в том же файле:
   ```rust
   #[cfg(test)]
   mod tests {
       #[test]
       fn test_something() {
           assert_eq!(2 + 2, 4);
       }
   }
   ```

2. Integration тесты - в `tests/`:
   ```rust
   // tests/my_test.rs
   use file_finder::scanner;
   
   #[test]
   fn test_scan() {
       // ...
   }
   ```

## Отладка

### VSCode launch.json
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug",
            "cargo": {
                "args": ["build", "--bin=file_finder"]
            },
            "args": ["--min-size", "100MB"],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

### Логирование
Добавьте в Cargo.toml:
```toml
[dependencies]
log = "0.4"
env_logger = "0.11"
```

В коде:
```rust
use log::{info, warn, error};

fn main() {
    env_logger::init();
    info!("Starting scan...");
}
```

Запуск с логами:
```bash
RUST_LOG=info cargo run
```

---

Эта структура обеспечивает модульность, тестируемость и легкость расширения проекта.
