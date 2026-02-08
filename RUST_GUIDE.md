# Шпаргалка по Rust для FileFinder

Это руководство поможет вам понять код проекта, если вы новичок в Rust.

## Основные концепции

### 1. Владение (Ownership)

Rust использует систему владения для управления памятью без сборщика мусора.

```rust
// String владеет данными
let s1 = String::from("hello");

// s1 перемещается в s2, s1 больше недействителен
let s2 = s1;

// println!("{}", s1); // ❌ ОШИБКА! s1 больше не владеет данными
println!("{}", s2); // ✅ OK
```

### 2. Заимствование (Borrowing)

```rust
fn calculate_length(s: &String) -> usize {  // &String - заимствование
    s.len()
}

let s1 = String::from("hello");
let len = calculate_length(&s1);  // Передаем ссылку
println!("s1: {}, len: {}", s1, len);  // s1 все еще действителен
```

**Правила:**
- Можно иметь либо одну изменяемую ссылку (&mut T)
- Либо любое количество неизменяемых ссылок (&T)
- Но не то и другое одновременно

### 3. Option и Result

#### Option<T> - может содержать значение или нет
```rust
fn find_user(id: u32) -> Option<User> {
    if id == 1 {
        Some(User { name: "Alice" })
    } else {
        None
    }
}

// Использование:
match find_user(1) {
    Some(user) => println!("Found: {}", user.name),
    None => println!("Not found"),
}

// Или короче:
if let Some(user) = find_user(1) {
    println!("Found: {}", user.name);
}
```

#### Result<T, E> - успех или ошибка
```rust
fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

// Использование:
match read_file("test.txt") {
    Ok(content) => println!("{}", content),
    Err(e) => println!("Error: {}", e),
}

// Или с оператором ?
fn process() -> Result<(), std::io::Error> {
    let content = read_file("test.txt")?;  // Автоматически возвращает Err
    println!("{}", content);
    Ok(())
}
```

## Специфичные для проекта концепции

### 4. Derive макросы

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
}
```

**Что это значит:**
- `Debug` - можно напечатать с `{:?}`
- `Clone` - можно клонировать с `.clone()`
- `Serialize` - можно сохранить в JSON
- `Deserialize` - можно загрузить из JSON

### 5. Атрибуты Clap (CLI аргументы)

```rust
#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "100MB")]
    min_size: String,
    
    #[arg(short, long)]
    output: Option<PathBuf>,
}
```

**Означает:**
- `-m` или `--min-size` для min_size
- По умолчанию "100MB"
- `-o` или `--output` опционально
- `Option<PathBuf>` значит может быть None

### 6. Match выражения

```rust
match choice {
    0 => {
        // Открыть расположение
        utils::open_file_location(&file.path)?;
    }
    1 => {
        // Удалить файл
        utils::delete_file(&file.path)?;
    }
    _ => {}  // Все остальные случаи
}
```

`_` - это wildcard pattern (любое другое значение)

### 7. Замыкания (Closures)

```rust
let files: Vec<&FileInfo> = all_files
    .iter()
    .filter(|f| f.size > 1000)  // Замыкание
    .collect();
```

`|f|` - параметры замыкания (как аргументы функции)

### 8. Итераторы

```rust
let total_size: u64 = files
    .iter()              // Создать итератор
    .map(|f| f.size)     // Преобразовать каждый элемент
    .sum();              // Суммировать
```

**Полезные методы:**
- `.filter(|x| условие)` - оставить элементы, где условие истинно
- `.map(|x| преобразование)` - преобразовать каждый элемент
- `.collect()` - собрать в коллекцию
- `.sum()` - суммировать
- `.any(|x| условие)` - проверить, есть ли хотя бы один
- `.all(|x| условие)` - проверить, все ли подходят

### 9. Векторы (Vec)

```rust
let mut files = Vec::new();     // Создать пустой вектор
files.push(file_info);          // Добавить элемент
files.len();                    // Количество элементов
files.sort_by(|a, b| b.size.cmp(&a.size));  // Сортировка
```

### 10. HashMap

```rust
use std::collections::HashMap;

let mut hash_map: HashMap<String, Vec<FileInfo>> = HashMap::new();

hash_map.insert(hash.clone(), vec![file]);
hash_map.entry(hash).or_insert_with(Vec::new).push(file);
```

### 11. Многопоточность

#### Arc (Atomic Reference Counted)
```rust
let counter = Arc::new(AtomicU64::new(0));
let counter_clone = counter.clone();  // Клонируем для другого потока

// Теперь counter и counter_clone указывают на одни данные
```

#### AtomicU64
```rust
counter.fetch_add(1, Ordering::Relaxed);  // Атомарное увеличение
let value = counter.load(Ordering::Relaxed);  // Атомарное чтение
```

#### rayon - параллельная обработка
```rust
use rayon::prelude::*;

files.par_iter_mut().for_each(|file| {
    // Обработка в параллель
    file.hash = Some(calculate_hash(&file.path));
});
```

### 12. Обработка ошибок

#### with_context - добавление контекста
```rust
use anyhow::{Context, Result};

let file = File::open(path)
    .with_context(|| format!("Не удалось открыть файл: {:?}", path))?;
```

#### bail - ранний выход с ошибкой
```rust
if selections.is_empty() {
    anyhow::bail!("Не выбрано ни одного диска");
}
```

### 13. PathBuf и Path

```rust
use std::path::{Path, PathBuf};

// PathBuf - владеющий тип (как String)
let mut path_buf = PathBuf::from("C:\\");
path_buf.push("Users");
path_buf.push("file.txt");

// Path - заимствованный тип (как &str)
let path: &Path = &path_buf;

// Операции:
path.extension();      // Расширение файла
path.file_name();      // Имя файла
path.parent();         // Родительская директория
path.exists();         // Проверка существования
```

### 14. Строки

```rust
// String - владеющая, изменяемая
let mut s = String::from("hello");
s.push_str(" world");

// &str - ссылка на строку, неизменяемая
let s: &str = "hello";

// Конверсия:
let string: String = "hello".to_string();
let str_ref: &str = &string;
```

### 15. Условная компиляция

```rust
#[cfg(windows)]
pub fn get_drives() -> Result<Vec<String>> {
    // Код только для Windows
}

#[cfg(not(windows))]
pub fn get_drives() -> Result<Vec<String>> {
    // Код для не-Windows
}
```

### 16. Модули

```rust
// В lib.rs или main.rs
mod models;    // Ищет models.rs или models/mod.rs
mod scanner;

// Использование:
use crate::models::FileInfo;
use crate::scanner::scan_files;
```

## Типичные паттерны в проекте

### Создание конфигурации с builder pattern
```rust
let config = ScanConfig::new(paths, min_size)
    .with_extensions(vec!["mp4".to_string()]);
```

### Обработка результатов с ?
```rust
fn process_file() -> Result<()> {
    let content = read_file("test.txt")?;  // Если ошибка, вернется Err
    let parsed = parse_content(&content)?;
    save_result(parsed)?;
    Ok(())  // Если все ОК, возвращаем Ok(())
}
```

### Форматирование строк
```rust
format!("Файл: {} размером {}", name, size)  // Создает String
println!("Файл: {} размером {}", name, size)  // Печатает

// С debug форматированием:
println!("Debug: {:?}", object);
```

## Полезные команды Cargo

```bash
cargo build             # Сборка (debug)
cargo build --release   # Сборка (оптимизированная)
cargo run               # Сборка и запуск
cargo test              # Запуск тестов
cargo check             # Проверка без сборки (быстро)
cargo fmt               # Форматирование кода
cargo clippy            # Линтер (подсказки по улучшению кода)
cargo doc --open        # Создать и открыть документацию
```

## Распространенные ошибки и решения

### "value borrowed here after move"
```rust
// ❌ Неправильно
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1);  // Ошибка!

// ✅ Правильно - клонирование
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{}", s1);

// ✅ Или заимствование
let s1 = String::from("hello");
let s2 = &s1;
println!("{}", s1);
```

### "cannot borrow as mutable"
```rust
// ❌ Неправильно
let s = String::from("hello");
s.push_str(" world");  // Ошибка! s не mut

// ✅ Правильно
let mut s = String::from("hello");
s.push_str(" world");
```

### "mismatched types"
```rust
// ❌ Неправильно
fn get_size() -> u64 {
    1024  // ОК
}

fn get_name() -> String {
    "hello"  // Ошибка! Это &str, а не String
}

// ✅ Правильно
fn get_name() -> String {
    "hello".to_string()
    // или
    String::from("hello")
}
```

## Чтение документации

```bash
# Документация для библиотеки
cargo doc --open

# Онлайн документация Rust
https://doc.rust-lang.org/std/

# Документация crates
https://docs.rs/
```

## Рекомендуемые ресурсы для изучения

1. **The Rust Book** - https://doc.rust-lang.org/book/
   - Официальная книга по Rust (есть на русском)

2. **Rust by Example** - https://doc.rust-lang.org/rust-by-example/
   - Изучение через примеры

3. **Rustlings** - https://github.com/rust-lang/rustlings
   - Интерактивные упражнения

4. **Rust Cookbook** - https://rust-lang-nursery.github.io/rust-cookbook/
   - Готовые решения типичных задач

## Полезные VSCode расширения

1. **rust-analyzer** - Автодополнение, проверка ошибок
2. **CodeLLDB** - Отладчик
3. **Better TOML** - Подсветка синтаксиса для Cargo.toml
4. **crates** - Управление зависимостями

## Советы по отладке

```rust
// Вывод для отладки
println!("Debug: {:?}", variable);
println!("Pretty debug: {:#?}", variable);

// Точка останова для отладчика
dbg!(&variable);

// Паника с сообщением
panic!("Something went wrong!");

// Assert для тестов
assert_eq!(actual, expected);
assert!(condition, "error message");
```

## Быстрая справка по синтаксису

```rust
let x = 5;              // Неизменяемая переменная
let mut x = 5;          // Изменяемая переменная
const MAX: u32 = 100;   // Константа

fn add(a: i32, b: i32) -> i32 {  // Функция
    a + b  // Возврат без return
}

struct Point {          // Структура
    x: f64,
    y: f64,
}

enum Status {           // Перечисление
    Ok,
    Error(String),
}

impl Point {            // Методы для структуры
    fn distance(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Цикл
for item in collection {
    // ...
}

while condition {
    // ...
}

loop {
    // ...
    break;
}

// if/else
if condition {
    // ...
} else if other_condition {
    // ...
} else {
    // ...
}
```

---

Это должно дать вам хорошую основу для понимания кода FileFinder!
