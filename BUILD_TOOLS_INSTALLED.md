# Build Tools установлены, но не работают - Решение

## Проблема

У вас установлены Visual Studio Build Tools, но:
- `where link.exe` ничего не показывает
- Cargo не может найти компилятор
- Ошибка "cannot open file 'msvcrt.lib'"

**Причина**: Переменные окружения не настроены (PATH, LIB и т.д.)

---

## Быстрое решение

### Способ 1: Использовать автоматический build (РЕКОМЕНДУЕТСЯ)

Просто запустите:
```cmd
build_auto.bat
```

Этот скрипт:
- Автоматически найдет Visual Studio
- Настроит окружение
- Соберет проект

**Готово!** Больше ничего делать не нужно.

---

### Способ 2: Настроить окружение вручную

1. Запустите:
```cmd
setup_vs_env.bat
```

2. Откроется новое окно командной строки с настроенным окружением

3. В этом окне запустите:
```cmd
cargo build --release
```

---

### Способ 3: Использовать Developer Command Prompt

1. **Нажмите Win + S** (поиск)

2. **Найдите**: "Developer Command Prompt for VS"
   - Полное название может быть:
     - "Developer Command Prompt for VS 2022"
     - "x64 Native Tools Command Prompt for VS 2022"
     - "Developer Command Prompt for VS 2019"

3. **Запустите** эту командную строку

4. **Перейдите** в папку проекта:
```cmd
cd C:\путь\к\file_finder
```

5. **Соберите**:
```cmd
cargo build --release
```

---

### Способ 4: Использовать PowerShell с автонастройкой

Создайте файл `build.ps1`:

```powershell
# Найти Visual Studio
$vsPath = & "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe" `
    -latest -property installationPath

if ($vsPath) {
    # Загрузить окружение VS
    & "$vsPath\Common7\Tools\Launch-VsDevShell.ps1"
    
    # Собрать проект
    cargo build --release
} else {
    Write-Host "Visual Studio not found!"
}
```

Запустите:
```powershell
powershell -ExecutionPolicy Bypass -File build.ps1
```

---

## Проверка что Build Tools правильно установлены

### Откройте Visual Studio Installer

1. **Win + S** → найдите "Visual Studio Installer"

2. Вы должны увидеть одну из:
   - Visual Studio 2022 (любая редакция)
   - Visual Studio 2019
   - Visual Studio Build Tools

3. **Нажмите "Modify"** (Изменить)

4. **Проверьте** что установлен компонент:
   - ✅ **"Desktop development with C++"**
   
   Если галочки нет - поставьте и нажмите "Modify"

---

## Постоянное решение (настроить PATH)

Если хотите чтобы работало везде, добавьте в PATH:

### Найдите путь к vcvars64.bat

Обычно это что-то вроде:
```
C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat
```

или
```
C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat
```

### Вариант A: Автоматическая настройка при каждом запуске CMD

Добавьте в конец файла `%USERPROFILE%\autoexec.bat`:

```batch
call "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul 2>nul
```

(Замените путь на ваш)

### Вариант B: Добавить в PATH вручную (сложно, не рекомендуется)

Это требует добавления ~10 путей в системные переменные. Проще использовать Developer Command Prompt или build_auto.bat.

---

## Рекомендация

**Проще всего**: используйте `build_auto.bat` каждый раз когда нужно собрать проект.

Он автоматически:
- Найдет Visual Studio
- Настроит окружение
- Соберет проект

Никаких ручных настроек не нужно!

---

## Проверка что всё работает

После использования любого из способов выше, проверьте:

```cmd
where link.exe
```

Должен показать путь типа:
```
C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.xx\bin\Hostx64\x64\link.exe
```

Если показывает - всё готово, можно собирать!

---

## Итого - что использовать

| Способ | Когда использовать |
|--------|-------------------|
| `build_auto.bat` | **Каждый раз** для сборки (самое простое) |
| `setup_vs_env.bat` | Если нужно окно с настроенным окружением |
| Developer Command Prompt | Если не доверяете скриптам |
| PowerShell скрипт | Если предпочитаете PowerShell |

**Мой совет**: просто используйте `build_auto.bat` и забудьте про проблему! 🎉
