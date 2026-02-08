# FileFinder - Шпаргалка команд

## 🔨 Сборка

```bash
# Windows
build.bat

# Linux/Mac
chmod +x build.sh && ./build.sh

# Или вручную
cargo build --release

# Для разработки (быстрая сборка)
cargo build
```

## 🚀 Основные команды

```bash
# Справка
file_finder --help

# Интерактивный режим (выбор дисков)
file_finder

# Сканирование конкретного диска
file_finder -p C:\

# Указать минимальный размер
file_finder --min-size 500MB
file_finder -m 1GB

# Фильтр по расширениям
file_finder -e mp4,mkv,avi
file_finder --extensions pdf,docx,xlsx

# Несколько дисков
file_finder -p C:\ -p D:\ -p E:\
```

## 💎 Полезные комбинации

```bash
# Найти большие видео
file_finder --min-size 500MB -e mp4,mkv,avi,mov

# Найти большие архивы
file_finder -m 100MB -e zip,rar,7z,tar,gz

# Найти ISO образы
file_finder -m 500MB -e iso,img

# Очистка Downloads
file_finder -p "C:\Users\YourName\Downloads" -m 10MB

# Быстрое сканирование только гигантских файлов
file_finder -m 5GB
```

## 🔍 Поиск дубликатов

```bash
# Найти дубликаты
file_finder --duplicates -m 100MB

# Дубликаты видео файлов
file_finder -d -e mp4,mkv -m 200MB

# Дубликаты с сохранением
file_finder --duplicates -m 500MB -o duplicates.json
```

## 💾 Сохранение/Загрузка

```bash
# Сохранить результаты
file_finder -m 500MB -o results.json
file_finder --output scan_results.json

# Загрузить результаты
file_finder -l results.json
file_finder --load scan_results.json

# Загрузить без интерактивного меню
file_finder --load results.json --no-interactive
```

## 🎛️ Опции

```bash
# Размер страницы
file_finder --page-size 50

# Без интерактивного режима
file_finder --no-interactive -o results.json

# Версия
file_finder --version

# Подробная справка
file_finder --help
```

## 📊 Примеры workflow

### Очистка диска C:
```bash
# 1. Найти большие файлы
file_finder -p C:\ -m 1GB -o big_files.json

# 2. Просмотреть и удалить
file_finder -l big_files.json
```

### Организация медиа
```bash
# 1. Инвентаризация
file_finder -p D:\Videos -e mp4,mkv,avi -m 50MB -o videos.json

# 2. Поиск дубликатов
file_finder -p D:\Videos -e mp4,mkv,avi --duplicates

# 3. Удаление дубликатов из интерактивного меню
```

### Регулярное сканирование
```bash
# Сохранить в файл с датой
file_finder -p C:\ -m 500MB -o "scan_2024-01-31.json" --no-interactive
```

## 🔧 Команды разработчика

```bash
# Проверка без сборки
cargo check

# Запуск тестов
cargo test

# Запуск с аргументами
cargo run -- --min-size 100MB

# Форматирование кода
cargo fmt

# Линтер
cargo clippy

# Документация
cargo doc --open

# Очистка
cargo clean
```

## 📏 Форматы размеров

```bash
file_finder -m 100        # 100 байт
file_finder -m 100KB      # 100 килобайт
file_finder -m 100MB      # 100 мегабайт
file_finder -m 1GB        # 1 гигабайт
file_finder -m 1GiB       # 1 гибибайт (1024 MB)
```

## 🎯 Расширения файлов

### Видео
```bash
-e mp4,mkv,avi,mov,wmv,flv,webm,m4v
```

### Аудио
```bash
-e mp3,wav,flac,aac,ogg,wma,m4a
```

### Изображения
```bash
-e jpg,jpeg,png,gif,bmp,svg,webp,ico
```

### Документы
```bash
-e pdf,doc,docx,xls,xlsx,ppt,pptx,txt
```

### Архивы
```bash
-e zip,rar,7z,tar,gz,bz2
```

### ISO
```bash
-e iso,img
```

## ⌨️ Алиасы

### Windows (PowerShell)
Добавьте в `$PROFILE`:
```powershell
function ff { file_finder.exe $args }
function ffd { file_finder.exe --duplicates $args }
function ffs { file_finder.exe --min-size 500MB $args }
```

### Linux/Mac
Добавьте в `~/.bashrc` или `~/.zshrc`:
```bash
alias ff='file_finder'
alias ffd='file_finder --duplicates'
alias ffs='file_finder --min-size 500MB'
```

## 🔥 Горячие примеры

```bash
# Top 10 самых больших файлов
file_finder -m 1GB --page-size 10

# Все видео на компьютере
file_finder -e mp4,mkv,avi -m 100MB

# Поиск возможных дубликатов для экономии места
file_finder -m 500MB --duplicates

# Анализ конкретной папки
file_finder -p "D:\Projects" -m 50MB

# Быстрое сканирование без UI
file_finder -m 1GB --no-interactive -o quick_scan.json
```

## 💡 Советы

1. **Для первого запуска**: просто `file_finder`
2. **Для быстрого сканирования**: увеличьте `--min-size`
3. **Для точного поиска**: используйте `-e` фильтр
4. **Для экономии места**: используйте `--duplicates`
5. **Для автоматизации**: используйте `--no-interactive -o file.json`

## 📱 Интерактивное меню

После сканирования:
- `Space` - выбор элемента
- `Enter` - подтвердить
- `↑/↓` - навигация
- В меню файлов:
  - Выбрать файл → действия
  - Предыдущая/Следующая страница
  - Фильтр
  - Выход

## 🚨 Troubleshooting

```bash
# "Access Denied"
→ Запустите от администратора

# Долгое сканирование
→ Увеличьте --min-size или используйте -e

# "cargo not found"
→ Установите Rust с https://rustup.rs/

# Ошибка компиляции
→ cargo clean && cargo build --release
```

---

**Сохраните эту шпаргалку для быстрого доступа!**
