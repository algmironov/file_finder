# FileFinder - Быстрый старт

Это краткое руководство поможет вам начать работу с FileFinder за 5 минут.

## Шаг 1: Установка Rust (если еще не установлен)

### Windows
1. Скачайте установщик: https://rustup.rs/
2. Запустите `rustup-init.exe`
3. Следуйте инструкциям (просто нажимайте Enter)
4. Перезапустите терминал

### Linux/Mac
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Проверка установки
```bash
rustc --version
cargo --version
```

Должны увидеть версии (например, `rustc 1.75.0`)

## Шаг 2: Сборка проекта

```bash
# Перейдите в папку проекта
cd file_finder

# Соберите проект (займет 2-5 минут при первом запуске)
cargo build --release

# Исполняемый файл будет в:
# Windows: target\release\file_finder.exe
# Linux/Mac: target/release/file_finder
```

### Альтернатива: Использование скриптов

**Windows:**
```bash
build.bat
```

**Linux/Mac:**
```bash
chmod +x build.sh
./build.sh
```

## Шаг 3: Первый запуск

### Запуск с помощью Cargo (из папки проекта)
```bash
# Справка
cargo run -- --help

# Первый запуск с интерактивным выбором дисков
cargo run

# С конкретными параметрами
cargo run -- --min-size 500MB
```

### Запуск скомпилированного файла

**Windows:**
```cmd
# Из папки проекта
target\release\file_finder.exe

# Или скопируйте в удобное место
copy target\release\file_finder.exe C:\Tools\
C:\Tools\file_finder.exe
```

**Linux/Mac:**
```bash
./target/release/file_finder

# Или установите в систему
sudo cp target/release/file_finder /usr/local/bin/
file_finder
```

## Шаг 4: Базовое использование

### Сценарий 1: Найти большие файлы на системном диске

```bash
# Windows
file_finder.exe -p C:\ --min-size 500MB

# Linux
file_finder -p / --min-size 500MB
```

Программа:
1. Покажет прогресс-бар сканирования
2. Выведет статистику
3. Откроет интерактивное меню с результатами

### Сценарий 2: Найти большие видео файлы

```bash
file_finder.exe --min-size 200MB -e mp4,mkv,avi
```

### Сценарий 3: Поиск дубликатов

```bash
file_finder.exe --min-size 100MB --duplicates
```

Программа найдет одинаковые файлы и предложит удалить копии.

### Сценарий 4: Сохранить результаты

```bash
file_finder.exe --min-size 500MB -o scan_results.json
```

Потом можно загрузить:
```bash
file_finder.exe --load scan_results.json
```

## Шаг 5: Интерактивное меню

После сканирования откроется интерактивное меню:

```
===================================================================================
Найденные файлы (страница 1/5) | Всего: 87
===================================================================================
#    📁   Имя файла                                      Размер          Путь
-----------------------------------------------------------------------------------
1    🎬   big_movie.mp4                                  4.2 GB          C:\Videos
2    📦   backup_2024.zip                                2.1 GB          C:\Backups
3    💿   windows.iso                                    5.5 GB          C:\ISOs
...
```

**Навигация:**
- Выберите файл для действий
- Перейдите на следующую/предыдущую страницу
- Примените фильтр
- Выход

**Действия с файлами:**
- 📂 Открыть расположение в проводнике
- 🗑️ Удалить файл
- ℹ️ Показать детали

## Типичные команды

### Очистка диска C:
```bash
file_finder.exe -p C:\ --min-size 1GB
```

### Анализ Downloads папки
```bash
file_finder.exe -p "C:\Users\YourName\Downloads" --min-size 10MB
```

### Поиск старых видео файлов
```bash
file_finder.exe -p D:\Videos -e mp4,mkv,avi --min-size 500MB
```

### Быстрое сканирование без интерактивного режима
```bash
file_finder.exe -p C:\ --min-size 1GB --no-interactive -o results.json
```

### Поиск всех больших архивов
```bash
file_finder.exe --min-size 100MB -e zip,rar,7z
```

## Распространенные проблемы

### "Access Denied" ошибки
**Решение**: Запустите от имени администратора

**Windows:**
```cmd
# Правый клик на PowerShell/CMD → Запуск от имени администратора
file_finder.exe -p C:\
```

### Долгое сканирование
**Решение**: Увеличьте минимальный размер или используйте фильтр

```bash
# Вместо --min-size 10MB
file_finder.exe --min-size 1GB

# Или с фильтром
file_finder.exe --min-size 100MB -e mp4,mkv
```

### Ошибка компиляции
**Решение**: Обновите Rust

```bash
rustup update
cargo clean
cargo build --release
```

### "cargo: command not found"
**Решение**: Перезапустите терминал после установки Rust или добавьте в PATH:

**Windows:**
```cmd
set PATH=%PATH%;%USERPROFILE%\.cargo\bin
```

**Linux/Mac:**
```bash
export PATH="$HOME/.cargo/bin:$PATH"
# Добавьте в ~/.bashrc или ~/.zshrc для постоянного эффекта
```

## Полезные советы

### 1. Создайте алиас для частого использования

**Windows (PowerShell):**
```powershell
# Добавьте в $PROFILE
function ff { file_finder.exe $args }

# Теперь можно использовать:
ff --min-size 500MB
```

**Linux/Mac:**
```bash
# Добавьте в ~/.bashrc или ~/.zshrc
alias ff='file_finder'

# Теперь можно использовать:
ff --min-size 500MB
```

### 2. Регулярное сканирование

Создайте batch/shell скрипт для регулярного запуска:

**Windows (weekly_scan.bat):**
```batch
@echo off
set DATE=%date:~-4,4%%date:~-10,2%%date:~-7,2%
file_finder.exe -p C:\ --min-size 500MB -o "scans\scan_%DATE%.json" --no-interactive
```

Добавьте в планировщик задач Windows.

**Linux (cron):**
```bash
# Добавьте в crontab
0 0 * * 0 /usr/local/bin/file_finder -p / --min-size 500MB -o ~/scans/scan_$(date +\%Y\%m\%d).json --no-interactive
```

### 3. Экспорт в Excel

Сохраните в JSON, затем импортируйте в Excel:
```bash
file_finder.exe --min-size 100MB -o results.json

# В Excel: Данные → Получить данные → Из файла → Из JSON
```

### 4. Комбинирование с другими инструментами

```bash
# Windows: Найти и открыть в проводнике
file_finder.exe --min-size 1GB --no-interactive -o big_files.json

# Linux: Найти и передать в другую команду
file_finder --min-size 1GB -o big_files.json
cat big_files.json | jq '.files[].path'
```

## Следующие шаги

1. **Изучите примеры**: См. `EXAMPLES.md` для более сложных сценариев
2. **Настройте под себя**: Отредактируйте код для своих нужд
3. **Создайте installer**: См. `DISTRIBUTION.md` для распространения
4. **Автоматизируйте**: Настройте регулярное сканирование

## Получение помощи

```bash
# Встроенная справка
file_finder.exe --help

# Версия
file_finder.exe --version

# Примеры
file_finder.exe --help | grep example
```

## Документация

- **README.md** - Полная документация
- **EXAMPLES.md** - Примеры команд
- **RUST_GUIDE.md** - Если хотите модифицировать код
- **PROJECT_STRUCTURE.md** - Структура проекта
- **IMPROVEMENTS.md** - Идеи для улучшений
- **DISTRIBUTION.md** - Как распространять

## Обратная связь

Если найдете баги или у вас есть идеи:
- Создайте Issue на GitHub
- Отправьте Pull Request
- Напишите на email

---

**Поздравляем! Вы готовы использовать FileFinder! 🎉**

Начните с простого:
```bash
file_finder.exe
```

И исследуйте возможности постепенно.
