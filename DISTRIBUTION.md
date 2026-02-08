# Руководство по распространению FileFinder

Это руководство описывает различные способы распространения вашего приложения FileFinder.

## 1. Простая портативная версия (Рекомендуется для начала)

### Подготовка
```bash
# Сборка оптимизированной версии
cargo build --release

# Исполняемый файл находится в:
# target/release/file_finder.exe (Windows)
# target/release/file_finder (Linux/Mac)
```

### Распространение
- Просто скопируйте `file_finder.exe` куда угодно
- Никаких зависимостей не требуется (статически скомпилирован)
- Можно запускать с флешки

### Уменьшение размера

#### Опция 1: Strip (встроенная)
Уже настроено в Cargo.toml: `strip = true`

#### Опция 2: UPX компрессия
```bash
# Скачайте UPX: https://upx.github.io/
upx --best --lzma target/release/file_finder.exe

# Размер уменьшится примерно в 2-3 раза
# Было: ~6-8 MB → Стало: ~2-3 MB
```

**Внимание**: Некоторые антивирусы могут ложно срабатывать на UPX

## 2. Создание ZIP архива

### Структура
```
FileFinder-v1.0-windows-x64.zip
├── file_finder.exe
├── README.txt (упрощенная версия)
├── EXAMPLES.txt (примеры команд)
└── LICENSE.txt
```

### Скрипт создания (Windows)
```batch
@echo off
set VERSION=1.0.0
set ARCH=windows-x64

cargo build --release

mkdir dist
copy target\release\file_finder.exe dist\
copy README.md dist\README.txt
copy EXAMPLES.md dist\EXAMPLES.txt
copy LICENSE dist\LICENSE.txt

cd dist
tar -a -c -f FileFinder-v%VERSION%-%ARCH%.zip *
cd ..

echo Archive created: dist/FileFinder-v%VERSION%-%ARCH%.zip
```

## 3. Windows Installer (MSI) с WiX Toolset

### Установка WiX
```bash
# Скачайте: https://wixtoolset.org/
# Или через winget:
winget install WiXToolset.WiX
```

### Создайте файл installer.wxs
```xml
<?xml version="1.0" encoding="UTF-8"?>
<Wix xmlns="http://schemas.microsoft.com/wix/2006/wi">
  <Product Id="*" 
           Name="FileFinder" 
           Language="1033" 
           Version="1.0.0" 
           Manufacturer="Your Name" 
           UpgradeCode="YOUR-GUID-HERE">
    
    <Package InstallerVersion="200" Compressed="yes" InstallScope="perMachine" />
    
    <MajorUpgrade DowngradeErrorMessage="A newer version is already installed." />
    <MediaTemplate EmbedCab="yes" />

    <Feature Id="ProductFeature" Title="FileFinder" Level="1">
      <ComponentGroupRef Id="ProductComponents" />
    </Feature>

    <Directory Id="TARGETDIR" Name="SourceDir">
      <Directory Id="ProgramFilesFolder">
        <Directory Id="INSTALLFOLDER" Name="FileFinder" />
      </Directory>
      <Directory Id="ProgramMenuFolder">
        <Directory Id="ApplicationProgramsFolder" Name="FileFinder"/>
      </Directory>
    </Directory>

    <DirectoryRef Id="INSTALLFOLDER">
      <Component Id="ProductComponent" Guid="YOUR-COMPONENT-GUID">
        <File Id="FileFinderExe" Source="target\release\file_finder.exe" KeyPath="yes" />
        <Environment Id="PATH" Name="PATH" Value="[INSTALLFOLDER]" Permanent="no" Part="last" Action="set" System="yes" />
      </Component>
    </DirectoryRef>

    <DirectoryRef Id="ApplicationProgramsFolder">
      <Component Id="ApplicationShortcut" Guid="YOUR-SHORTCUT-GUID">
        <Shortcut Id="ApplicationStartMenuShortcut" 
                  Name="FileFinder"
                  Target="[INSTALLFOLDER]file_finder.exe"
                  WorkingDirectory="INSTALLFOLDER"/>
        <RemoveFolder Id="ApplicationProgramsFolder" On="uninstall"/>
        <RegistryValue Root="HKCU" Key="Software\FileFinder" Name="installed" Type="integer" Value="1" KeyPath="yes"/>
      </Component>
    </DirectoryRef>

    <ComponentGroup Id="ProductComponents" Directory="INSTALLFOLDER">
      <ComponentRef Id="ProductComponent" />
    </ComponentGroup>
  </Product>
</Wix>
```

### Компиляция MSI
```bash
# Сгенерировать GUID для UpgradeCode и ComponentGuids
# Можно использовать: https://www.guidgenerator.com/

candle installer.wxs
light -out FileFinder-v1.0.0.msi installer.wixobj
```

## 4. Setup.exe с Inno Setup (Проще чем WiX)

### Установка Inno Setup
Скачайте с: https://jrsoftware.org/isinfo.php

### Создайте файл setup.iss
```inno
[Setup]
AppName=FileFinder
AppVersion=1.0.0
AppPublisher=Your Name
DefaultDirName={autopf}\FileFinder
DefaultGroupName=FileFinder
OutputDir=dist
OutputBaseFilename=FileFinder-Setup-v1.0.0
Compression=lzma2
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "russian"; MessagesFile: "compiler:Languages\Russian.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "addtopath"; Description: "Add to PATH"; GroupDescription: "Additional options:"

[Files]
Source: "target\release\file_finder.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "README.md"; DestDir: "{app}"; DestName: "README.txt"; Flags: isreadme
Source: "EXAMPLES.md"; DestDir: "{app}"; DestName: "EXAMPLES.txt"

[Icons]
Name: "{group}\FileFinder"; Filename: "{app}\file_finder.exe"
Name: "{group}\{cm:UninstallProgram,FileFinder}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\FileFinder"; Filename: "{app}\file_finder.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\file_finder.exe"; Parameters: "--help"; Description: "{cm:LaunchProgram,FileFinder}"; Flags: shellexec postinstall skipifsilent

[Code]
procedure CurStepChanged(CurStep: TSetupStep);
var
  ResultCode: Integer;
begin
  if (CurStep = ssPostInstall) and WizardIsTaskSelected('addtopath') then
  begin
    Exec('cmd.exe', '/c setx PATH "%PATH%;' + ExpandConstant('{app}') + '"', '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
  end;
end;
```

### Компиляция Setup.exe
```bash
# Откройте setup.iss в Inno Setup Compiler и нажмите Compile
# Или из командной строки:
"C:\Program Files (x86)\Inno Setup 6\ISCC.exe" setup.iss
```

## 5. Публикация на GitHub Releases

### Подготовка релиза
```bash
# 1. Создайте тег версии
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0

# 2. Соберите для разных платформ
# Windows
cargo build --release --target x86_64-pc-windows-msvc

# Linux
cargo build --release --target x86_64-unknown-linux-gnu

# macOS (если доступно)
cargo build --release --target x86_64-apple-darwin
```

### Создайте Release на GitHub
1. Перейдите в Releases → Create new release
2. Выберите тег v1.0.0
3. Загрузите:
   - `file_finder.exe` (Windows)
   - `file_finder` (Linux)
   - ZIP архивы
   - Installer (если создали)

### Пример Release Notes
```markdown
# FileFinder v1.0.0

## 🎉 Первый релиз!

Быстрая утилита для поиска больших файлов на Windows.

### ✨ Основные функции
- Параллельное сканирование файловой системы
- Интерактивный выбор дисков
- Поиск дубликатов по SHA-256
- Фильтрация по расширениям
- Сохранение результатов

### 📥 Установка

**Windows:**
- Скачайте `FileFinder-Setup-v1.0.0.exe` и запустите
- Или скачайте `FileFinder-v1.0.0-windows-x64.zip` и распакуйте

**Linux:**
- Скачайте `file_finder-linux-x64`
- Сделайте исполняемым: `chmod +x file_finder`

### 📖 Документация
См. [README.md](link) и [EXAMPLES.md](link)

### 🐛 Известные проблемы
Нет

### 🙏 Благодарности
Спасибо всем contributors!
```

## 6. Публикация на Cargo (для Rust-сообщества)

### Подготовка
```bash
# Убедитесь, что Cargo.toml заполнен
# Добавьте:
# - description
# - license
# - repository
# - keywords
# - categories
```

### Публикация
```bash
cargo login <your-token>
cargo publish
```

### Теперь можно устанавливать:
```bash
cargo install file_finder
```

## 7. Chocolatey Package (Windows)

### Создайте chocolateyinstall.ps1
```powershell
$ErrorActionPreference = 'Stop'

$packageName = 'file-finder'
$url = 'https://github.com/yourname/file_finder/releases/download/v1.0.0/file_finder.exe'
$checksum = 'YOUR-SHA256-CHECKSUM'

$installDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

Get-ChocolateyWebFile -PackageName $packageName `
                       -FileFullPath "$installDir\file_finder.exe" `
                       -Url $url `
                       -Checksum $checksum `
                       -ChecksumType 'sha256'
```

### Установка через Chocolatey
```bash
choco install file-finder
```

## 8. Winget Package

### Создайте манифест для winget
```yaml
# file_finder.yaml
PackageIdentifier: YourName.FileFinder
PackageVersion: 1.0.0
PackageName: FileFinder
Publisher: Your Name
License: MIT
ShortDescription: Fast file finder utility
Installers:
  - Architecture: x64
    InstallerType: exe
    InstallerUrl: https://github.com/yourname/file_finder/releases/download/v1.0.0/FileFinder-Setup-v1.0.0.exe
    InstallerSha256: YOUR-SHA256-CHECKSUM
```

### Установка
```bash
winget install FileFinder
```

## 9. Автообновление

### Опция 1: Самообновление в коде
```rust
// Добавить в main.rs
#[arg(long)]
update: bool,

if args.update {
    check_and_update()?;
}

fn check_and_update() -> Result<()> {
    // Проверить GitHub Releases API
    // Скачать новую версию если доступна
    // Заменить текущий exe
}
```

### Опция 2: Отдельная программа updater
```rust
// updater/main.rs
// Маленькая программа, которая:
// 1. Скачивает новую версию
// 2. Закрывает file_finder.exe
// 3. Заменяет файл
// 4. Запускает обновленную версию
```

## 10. Подпись кода (Code Signing)

### Для Windows
```bash
# Нужен сертификат от Certificate Authority

signtool sign /f mycert.pfx /p password /t http://timestamp.digicert.com target\release\file_finder.exe
```

**Зачем нужно:**
- Windows не будет показывать "Unknown Publisher"
- Антивирусы меньше ругаются
- Больше доверия от пользователей

## 11. Статистика использования (опционально)

### Telemetry (с согласия пользователя)
```rust
// Анонимная статистика:
// - Версия программы
// - ОС и версия
// - Количество запусков
// - Популярные функции

// Только с явного согласия пользователя!
#[arg(long)]
enable_telemetry: bool,
```

## Рекомендации

### Для быстрого старта:
1. Соберите с `cargo build --release`
2. Создайте ZIP архив
3. Опубликуйте на GitHub Releases

### Для профессионального распространения:
1. Создайте Installer (Inno Setup или WiX)
2. Подпишите код
3. Публикуйте на Winget/Chocolatey
4. Настройте автообновление

### Для максимальной доступности:
1. Все вышеперечисленное
2. Публикация на Cargo
3. Создание GUI версии
4. Портирование на Linux/macOS

## Версионирование

Используйте [Semantic Versioning](https://semver.org/):
- **MAJOR** (1.x.x) - несовместимые изменения API
- **MINOR** (x.1.x) - новые функции (обратно совместимые)
- **PATCH** (x.x.1) - исправления багов

## Чек-лист перед релизом

- [ ] Все тесты проходят (`cargo test`)
- [ ] Обновлена версия в Cargo.toml
- [ ] Обновлен CHANGELOG.md
- [ ] Обновлен README.md
- [ ] Создан git tag
- [ ] Собрано для всех платформ
- [ ] Проверено антивирусами
- [ ] Создан installer
- [ ] Написаны Release Notes
- [ ] Обновлена документация

## Лицензия

Не забудьте добавить файл LICENSE:
```bash
# Для MIT License
curl https://raw.githubusercontent.com/licenses/license-templates/master/templates/mit.txt > LICENSE

# Замените [year] и [fullname]
```

## Обратная связь

Создайте каналы для обратной связи:
- GitHub Issues
- Email
- Discord/Telegram сообщество (опционально)
- Twitter/X для анонсов
