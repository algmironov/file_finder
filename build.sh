#!/bin/bash

echo "======================================="
echo "FileFinder Build Script"
echo "======================================="
echo ""

# Проверка наличия Rust
if ! command -v cargo &> /dev/null; then
    echo "[ERROR] Rust не установлен!"
    echo ""
    echo "Установите Rust с https://rustup.rs/"
    echo ""
    exit 1
fi

echo "[1/3] Проверка зависимостей..."
cargo --version
rustc --version
echo ""

echo "[2/3] Сборка проекта в режиме release..."
cargo build --release

if [ $? -ne 0 ]; then
    echo ""
    echo "[ERROR] Ошибка сборки!"
    exit 1
fi

echo ""
echo "[3/3] Успешно собрано!"
echo ""
echo "Исполняемый файл: target/release/file_finder"

# Получаем размер файла
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    SIZE=$(stat -f%z target/release/file_finder)
else
    # Linux
    SIZE=$(stat -c%s target/release/file_finder)
fi

echo "Размер: $SIZE bytes"

echo ""
echo "======================================="
echo "Сборка завершена!"
echo "======================================="
echo ""

# Спрашиваем, запустить ли программу
read -p "Запустить программу? (y/n) " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo ""
    echo "Запуск file_finder --help"
    echo ""
    ./target/release/file_finder --help
fi
