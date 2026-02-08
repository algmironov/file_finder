# Исправление ошибки "cannot open file 'msvcrt.lib'"

## Проблема

```
LINK : fatal error LNK1104: cannot open file 'msvcrt.lib'
```

**Причина**: Отсутствуют Visual Studio Build Tools, необходимые для компиляции Rust на Windows.

---

## Решение 1: Установка Visual Studio Build Tools (РЕКОМЕНДУЕТСЯ)

### Шаг 1: Скачайте установщик

**Вариант A (прямая ссылка):**
Скачайте Build Tools for Visual Studio 2022:
https://aka.ms/vs/17/release/vs_buildtools.exe

**Вариант B (через страницу загрузок):**
1. Перейдите: https://visualstudio.microsoft.com/downloads/
2. Прокрутите вниз до "Tools for Visual Studio"
3. Скачайте "Build Tools for Visual Studio 2022"

### Шаг 2: Запустите установщик

1. Запустите скачанный `vs_buildtools.exe`
2. В установщике выберите вкладку **"Workloads"**
3. Отметьте галочку: **"Desktop development with C++"**

   Это включит:
   - ✅ MSVC v143 - VS 2022 C++ x64/x86 build tools
   - ✅ Windows 11 SDK (или Windows 10 SDK)
   - ✅ C++ CMake tools
   - ✅ Testing tools

### Шаг 3: Установите

1. Нажмите "Install" (справа внизу)
2. Дождитесь окончания (может занять 10-30 минут, ~2-6 GB)
3. Перезагрузите компьютер

### Шаг 4: Проверьте

```cmd
"C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
```

Если команда отработала без ошибок - всё ОК!

### Шаг 5: Соберите проект

```cmd
cd file_finder
cargo clean
cargo build --release
```

---

## Решение 2: Использовать GNU toolchain (альтернатива)

Если не хотите устанавливать Visual Studio, можно использовать MinGW:

### Шаг 1: Установите MinGW toolchain

```cmd
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

### Шаг 2: Установите MinGW-w64

Скачайте и установите: https://www.mingw-w64.org/downloads/

Или через winget:
```cmd
winget install mingw-w64
```

### Шаг 3: Соберите проект

```cmd
cargo clean
cargo build --release
```

**Примечание**: GNU toolchain может быть немного медленнее и иметь проблемы с некоторыми Windows API.

---

## Решение 3: Использовать предустановленную Visual Studio

Если у вас уже установлена Visual Studio (любая версия):

### Проверьте компоненты:

1. Откройте Visual Studio Installer
2. Нажмите "Modify" на вашей версии
3. Убедитесь, что установлено:
   - ✅ Desktop development with C++
   - ✅ Windows 10/11 SDK

### Если компоненты отсутствуют:

1. Отметьте "Desktop development with C++"
2. Нажмите "Modify"
3. Дождитесь установки
4. Перезагрузите компьютер

---

## Быстрая диагностика

После установки проверьте:

```cmd
where link.exe
where cl.exe
```

Должны показать путь к компилятору, например:
```
C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.xx\bin\Hostx64\x64\link.exe
```

---

## Частые вопросы

### Q: Сколько места занимает?
A: Build Tools ~2-6 GB в зависимости от компонентов

### Q: Можно ли использовать старые версии VS?
A: Да, VS 2017, 2019, 2022 - все подойдут

### Q: Нужна ли полная Visual Studio?
A: Нет, достаточно Build Tools (бесплатно)

### Q: Что если у меня VS 2019 или 2017?
A: Тоже подойдёт, просто убедитесь что установлен "Desktop development with C++"

---

## После установки

### Перезапустите терминал

Обязательно откройте новое окно командной строки после установки!

### Соберите проект

```cmd
cd file_finder
cargo clean        # Очистить старые файлы
cargo build --release
```

### Если всё равно ошибка

Попробуйте запустить из "Developer Command Prompt for VS":
1. Пуск → Visual Studio 2022 → Developer Command Prompt
2. Перейдите в папку проекта
3. Запустите `cargo build --release`

---

## Проверка окружения

Создайте файл `test_env.bat`:

```batch
@echo off
echo === Checking Rust Environment ===
echo.
rustc --version
cargo --version
echo.
echo === Checking Visual Studio Tools ===
where link.exe
where cl.exe
echo.
echo === Checking Environment Variables ===
echo LIB=%LIB%
echo.
pause
```

Запустите его и проверьте вывод.

---

## Альтернатива: Использовать WSL

Если ничего не помогает, можно собрать в WSL (Windows Subsystem for Linux):

```bash
# В WSL (Ubuntu)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
cd /mnt/c/путь/к/проекту
cargo build --release
```

Исполняемый файл будет для Linux, но сборка проще.

---

## Итого

**Для большинства пользователей:**
1. Скачать: https://aka.ms/vs/17/release/vs_buildtools.exe
2. Установить: "Desktop development with C++"
3. Перезагрузить компьютер
4. Запустить: `cargo build --release`

**Время установки**: ~20-40 минут
**Размер**: ~2-6 GB
**Результат**: Полностью рабочая среда для Rust

---

## Полезные ссылки

- Visual Studio Downloads: https://visualstudio.microsoft.com/downloads/
- Rust Installation Guide: https://doc.rust-lang.org/book/ch01-01-installation.html
- Windows Prerequisites: https://rust-lang.github.io/rustup/installation/windows.html

---

Удачи! После установки всё должно заработать без проблем! 🎉
