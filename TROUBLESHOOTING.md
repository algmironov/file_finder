# Решение проблем со сборкой

## Проблема 1: Кракозябры (кириллица не отображается)

**Причина**: Неправильная кодировка консоли Windows

**Решение**: Уже исправлено в новом `build.bat`

Если всё равно видите символы:
1. Откройте PowerShell (вместо CMD)
2. Или используйте новый Windows Terminal
3. Или просто игнорируйте - работает одинаково

---

## Проблема 2: "requires rustc 1.80 or newer"

**Причина**: У вас Rust 1.77.1, нужен 1.80+

### Решение 1: Обновить Rust (РЕКОМЕНДУЕТСЯ)

**Вариант A - Используя скрипт:**
```cmd
update_rust.bat
```

**Вариант B - Вручную:**
```cmd
rustup update stable
```

Проверьте версию:
```cmd
rustc --version
```

Должно быть: `rustc 1.80.0` или выше

### Решение 2: Использовать старые версии библиотек (если обновление невозможно)

Отредактируйте `Cargo.toml` и измените:
```toml
rayon = "1.8"
```

на:
```toml
rayon = "1.10"
```

Уже исправлено в новой версии файла!

---

## Полная последовательность действий

### Способ 1 (простой):

```cmd
1. update_rust.bat      (обновить Rust)
2. build.bat            (собрать проект)
```

### Способ 2 (вручную):

```cmd
1. rustup update stable
2. rustc --version      (проверить: должно быть 1.80+)
3. cargo build --release
```

---

## Проверка успешной сборки

После успешной сборки вы увидите:
```
[3/3] Build successful!

Executable: target\release\file_finder.exe
Size: 7654321 bytes

=======================================
Build completed!
=======================================
```

И появится файл: `target\release\file_finder.exe`

---

## Если обновление Rust не работает

### Переустановите Rust полностью:

1. **Удалите старый Rust:**
   ```cmd
   rustup self uninstall
   ```

2. **Скачайте новую версию:**
   - Перейдите на https://rustup.rs/
   - Скачайте `rustup-init.exe`
   - Запустите установщик

3. **Проверьте версию:**
   ```cmd
   rustc --version
   ```

4. **Соберите проект:**
   ```cmd
   build.bat
   ```

---

## Альтернативный способ сборки

Если `build.bat` не работает, попробуйте напрямую:

```cmd
cargo clean
cargo build --release
```

Файл будет: `target\release\file_finder.exe`

---

## Частые ошибки

### "cargo: command not found"

**Решение**: Перезапустите командную строку после установки Rust

Или добавьте в PATH:
```cmd
set PATH=%PATH%;%USERPROFILE%\.cargo\bin
```

### "Blocking waiting for file lock"

**Решение**: 
1. Закройте все окна с cargo
2. Удалите папку: `%USERPROFILE%\.cargo\.package-cache`
3. Попробуйте снова

### "linker 'link.exe' failed" или "cannot open file 'msvcrt.lib'"

**Причина**: Отсутствуют Visual Studio Build Tools

**Решение**: 

**Шаг 1**: Проверьте окружение
```cmd
check_environment.bat
```

**Шаг 2**: Установите Build Tools
1. Скачайте: https://aka.ms/vs/17/release/vs_buildtools.exe
2. Запустите и выберите "Desktop development with C++"
3. Установите (~20-40 минут, 2-6 GB)
4. Перезагрузите компьютер
5. Запустите `build.bat`

**Подробнее**: См. `FIX_LINKER_ERROR.md`

**Альтернатива**: Используйте GNU toolchain
```cmd
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
cargo build --release
```

---

## Быстрая диагностика

Запустите эти команды и покажите результат:

```cmd
rustc --version
cargo --version
where cargo
where rustc
```

Должно быть примерно так:
```
rustc 1.80.0 (или выше)
cargo 1.80.0 (или выше)
C:\Users\YourName\.cargo\bin\cargo.exe
C:\Users\YourName\.cargo\bin\rustc.exe
```

---

## Контакты для помощи

Если ничего не помогает:
1. Скопируйте полный вывод ошибки
2. Запустите команды диагностики выше
3. Опишите проблему подробно

---

## После успешной сборки

Запустите программу:
```cmd
target\release\file_finder.exe --help
target\release\file_finder.exe
```

Готово! 🎉
