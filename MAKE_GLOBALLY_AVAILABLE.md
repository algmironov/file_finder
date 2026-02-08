# Как сделать утилиту доступной из любого места

После сборки у вас есть файл `target\release\file_finder.exe`, но чтобы запускать его из любой папки, нужно добавить в PATH или установить в систему.

---

## Способ 1: Добавить в PATH (РЕКОМЕНДУЕТСЯ)

### Вариант A - Скопировать в существующую папку из PATH

**Самый простой способ:**

```cmd
copy target\release\file_finder.exe C:\Windows\System32\
```

Теперь можно запускать откуда угодно:
```cmd
file_finder --min-size 500MB
```

**Альтернативные места:**
```cmd
REM В папку пользователя (не требует прав администратора)
mkdir %USERPROFILE%\bin
copy target\release\file_finder.exe %USERPROFILE%\bin\

REM В Program Files (требует права администратора)
copy target\release\file_finder.exe "C:\Program Files\FileFinder\"
```

### Вариант B - Добавить текущую папку в PATH

**Через PowerShell (временно, до перезагрузки):**
```powershell
$env:PATH += ";C:\Users\algmi\source\Rust\file_finder\target\release"
```

**Через PowerShell (постоянно, только для текущего пользователя):**
```powershell
# Добавить в PATH пользователя
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
$newPath = "$currentPath;C:\Users\algmi\source\Rust\file_finder\target\release"
[Environment]::SetEnvironmentVariable("Path", $newPath, "User")
```

**Через GUI (постоянно):**
1. Win + X → "Система"
2. "Дополнительные параметры системы"
3. "Переменные среды"
4. В разделе "Переменные пользователя" найдите `Path`
5. Нажмите "Изменить"
6. "Создать" → добавьте: `C:\Users\algmi\source\Rust\file_finder\target\release`
7. OK → OK → OK
8. **Перезапустите PowerShell/CMD**

### Вариант C - Через CMD (постоянно, для системы)

```cmd
REM Требует запуска CMD от администратора
setx PATH "%PATH%;C:\Users\algmi\source\Rust\file_finder\target\release" /M
```

---

## Способ 2: Создать алиас (только PowerShell)

### Временный алиас (до закрытия окна):
```powershell
function ff { & "C:\Users\algmi\source\Rust\file_finder\target\release\file_finder.exe" $args }
```

Теперь:
```powershell
ff --min-size 500MB
```

### Постоянный алиас:

**Шаг 1 - Создайте профиль PowerShell** (если его нет):
```powershell
if (!(Test-Path -Path $PROFILE)) {
    New-Item -ItemType File -Path $PROFILE -Force
}
```

**Шаг 2 - Откройте профиль:**
```powershell
notepad $PROFILE
```

**Шаг 3 - Добавьте в файл:**
```powershell
# FileFinder alias
function ff { 
    & "C:\Users\algmi\source\Rust\file_finder\target\release\file_finder.exe" $args 
}

# Или полное имя
function file_finder { 
    & "C:\Users\algmi\source\Rust\file_finder\target\release\file_finder.exe" $args 
}
```

**Шаг 4 - Сохраните и перезапустите PowerShell**

Теперь можно:
```powershell
ff --min-size 1GB
file_finder -e mp4,mkv
```

---

## Способ 3: Установить через Cargo

Если хотите установить "правильно" через Cargo:

```cmd
cd C:\Users\algmi\source\Rust\file_finder
cargo install --path .
```

Это установит в `%USERPROFILE%\.cargo\bin\file_finder.exe`

Эта папка обычно уже в PATH (добавляется при установке Rust).

Проверить:
```cmd
where file_finder
```

Теперь можно запускать откуда угодно:
```cmd
file_finder --min-size 500MB
```

**Чтобы обновить:**
```cmd
cargo install --path . --force
```

**Чтобы удалить:**
```cmd
cargo uninstall file_finder
```

---

## Способ 4: Создать BAT-файл в PATH

Создайте файл `file_finder.bat` в папке из PATH:

```batch
@echo off
"C:\Users\algmi\source\Rust\file_finder\target\release\file_finder.exe" %*
```

Сохраните в, например:
```
C:\Windows\System32\file_finder.bat
```

или
```
%USERPROFILE%\bin\file_finder.bat
```

Теперь можно запускать:
```cmd
file_finder --min-size 1GB
```

---

## Способ 5: Создать символическую ссылку

**Требует права администратора:**

```cmd
mklink "C:\Windows\System32\file_finder.exe" "C:\Users\algmi\source\Rust\file_finder\target\release\file_finder.exe"
```

Теперь `file_finder` доступен везде, но при пересборке изменения подтягиваются автоматически.

---

## Рекомендованный workflow

### Для разработки:
```cmd
# Установить через cargo
cargo install --path .

# После изменений в коде
cargo build --release
cargo install --path . --force
```

### Для использования:
```cmd
# Вариант 1: Просто скопировать в Windows\System32
copy target\release\file_finder.exe C:\Windows\System32\

# Вариант 2: Через cargo (чище)
cargo install --path .
```

---

## Проверка установки

После любого способа, проверьте:

**PowerShell:**
```powershell
Get-Command file_finder
```

Должно показать путь к exe.

**CMD:**
```cmd
where file_finder
```

Должно показать путь.

**Тест:**
```cmd
file_finder --version
file_finder --help
```

---

## Автоматическая установка

Создайте скрипт `install.bat`:

```batch
@echo off
echo =======================================
echo Installing FileFinder
echo =======================================
echo.

echo [1/2] Building release version...
cargo build --release

if %ERRORLEVEL% NEQ 0 (
    echo Build failed!
    pause
    exit /b 1
)

echo [2/2] Installing to system...

REM Проверяем наличие cargo bin в PATH
where cargo >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo Installing via cargo...
    cargo install --path . --force
    echo.
    echo [OK] Installed to: %USERPROFILE%\.cargo\bin\file_finder.exe
    goto :verify
)

REM Альтернатива - копирование в Windows
echo Installing to Windows\System32...
copy /Y target\release\file_finder.exe C:\Windows\System32\
echo [OK] Installed to: C:\Windows\System32\file_finder.exe

:verify
echo.
echo =======================================
echo Installation complete!
echo =======================================
echo.
echo Testing...
file_finder --version
echo.
echo You can now run 'file_finder' from anywhere!
echo.
pause
```

Запустите:
```cmd
install.bat
```

---

## Удаление

### Если установили через cargo:
```cmd
cargo uninstall file_finder
```

### Если скопировали в System32:
```cmd
del C:\Windows\System32\file_finder.exe
```

### Если добавили в PATH:
Удалите путь из переменных окружения.

---

## Примеры использования после установки

```powershell
# Из любой папки
cd ~
file_finder --min-size 1GB

cd C:\
file_finder -p D:\ -e mp4,mkv

cd Documents
file_finder --duplicates -m 500MB
```

---

## Частые вопросы

**Q: Что лучше - копировать в System32 или установить через cargo?**  
A: Через cargo чище, но копирование проще. Для личного использования - без разницы.

**Q: Нужны ли права администратора?**  
A: 
- Для System32 - да
- Для cargo install - нет
- Для добавления в PATH пользователя - нет
- Для добавления в PATH системы - да

**Q: Что если я пересоберу проект?**  
A: Нужно переустановить:
```cmd
cargo install --path . --force
```
или
```cmd
copy /Y target\release\file_finder.exe C:\Windows\System32\
```

**Q: Можно ли переименовать утилиту?**  
A: Да, просто переименуйте .exe файл при копировании:
```cmd
copy target\release\file_finder.exe C:\Windows\System32\ff.exe
```

**Q: Работает ли это с PowerShell Core (pwsh)?**  
A: Да, все способы работают.

---

## Мой выбор

**Для разработки:**
```cmd
cargo install --path .
```

**Для распространения другим:**
Создать installer (см. DISTRIBUTION.md)

**Для быстрого личного использования:**
```cmd
copy target\release\file_finder.exe C:\Windows\System32\
```

Выбирайте что вам удобнее! 🚀
