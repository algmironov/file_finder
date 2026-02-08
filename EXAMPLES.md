# Примеры использования FileFinder

## 1. Базовые сценарии

### Найти все большие файлы (>100MB) с интерактивным выбором дисков
```bash
file_finder.exe
```

### Найти файлы больше 500MB на диске C:
```bash
file_finder.exe -p C:\ --min-size 500MB
```

### Найти файлы больше 1GB на нескольких дисках
```bash
file_finder.exe -p C:\ -p D:\ -p E:\ --min-size 1GB
```

## 2. Работа с конкретными типами файлов

### Найти большие видео файлы
```bash
file_finder.exe --min-size 500MB -e mp4,mkv,avi,mov
```

### Найти большие архивы
```bash
file_finder.exe --min-size 100MB -e zip,rar,7z,tar,gz
```

### Найти большие документы
```bash
file_finder.exe --min-size 10MB -e pdf,docx,xlsx,pptx
```

### Найти ISO образы
```bash
file_finder.exe --min-size 500MB -e iso,img
```

### Найти большие аудио файлы
```bash
file_finder.exe --min-size 50MB -e mp3,flac,wav,m4a
```

## 3. Поиск дубликатов

### Найти дубликаты видео файлов
```bash
file_finder.exe -e mp4,mkv,avi --duplicates --min-size 100MB
```

### Найти дубликаты любых больших файлов
```bash
file_finder.exe --min-size 500MB --duplicates
```

### Найти дубликаты документов и сохранить результат
```bash
file_finder.exe -e pdf,docx --duplicates -o duplicates.json
```

## 4. Сохранение и загрузка результатов

### Сканировать и сохранить результаты
```bash
file_finder.exe -p C:\ --min-size 200MB -o scan_results.json
```

### Загрузить и просмотреть сохраненные результаты
```bash
file_finder.exe --load scan_results.json
```

### Загрузить без интерактивного меню (просто статистика)
```bash
file_finder.exe --load scan_results.json --no-interactive
```

## 5. Продвинутое использование

### Полное сканирование системы с поиском дубликатов и сохранением
```bash
file_finder.exe -p C:\ -p D:\ --min-size 100MB --duplicates -o full_scan.json
```

### Быстрое сканирование только больших файлов без интерактивного режима
```bash
file_finder.exe -p C:\ --min-size 1GB --no-interactive -o big_files.json
```

### Поиск конкретных типов с настроенной пагинацией
```bash
file_finder.exe -e mp4,mkv --min-size 200MB --page-size 50
```

### Комплексный анализ медиа файлов
```bash
file_finder.exe -p D:\Media -e mp4,mkv,avi,mp3,flac --min-size 50MB --duplicates -o media_analysis.json
```

## 6. Специализированные сценарии

### Очистка диска - найти самые большие файлы
```bash
file_finder.exe -p C:\ --min-size 1GB --page-size 10
# В интерактивном режиме можно удалить ненужные файлы
```

### Аудит медиа коллекции - найти дубликаты
```bash
file_finder.exe -p D:\Videos -e mp4,mkv,avi --duplicates
# Программа покажет группы дубликатов и предложит удалить копии
```

### Инвентаризация документов
```bash
file_finder.exe -p C:\Users -e pdf,doc,docx,xls,xlsx --min-size 1MB -o documents_inventory.json
```

### Поиск старых резервных копий
```bash
file_finder.exe -e bak,backup,old --min-size 100MB
```

### Анализ downloads папки
```bash
file_finder.exe -p C:\Users\YourName\Downloads --min-size 10MB -o downloads_analysis.json
```

## 7. Workflow примеры

### Пример 1: Освобождение места на диске
```bash
# Шаг 1: Найти все большие файлы
file_finder.exe -p C:\ --min-size 500MB -o large_files.json

# Шаг 2: Просмотреть результаты и удалить ненужное
file_finder.exe --load large_files.json

# Шаг 3: Найти дубликаты
file_finder.exe -p C:\ --min-size 100MB --duplicates
```

### Пример 2: Организация медиа библиотеки
```bash
# Шаг 1: Инвентаризация всех видео
file_finder.exe -p D:\Videos -e mp4,mkv,avi --min-size 50MB -o video_inventory.json

# Шаг 2: Найти дубликаты
file_finder.exe -p D:\Videos -e mp4,mkv,avi --duplicates

# Шаг 3: Анализ больших файлов низкого качества
file_finder.exe -p D:\Videos -e avi --min-size 1GB
```

### Пример 3: Регулярный мониторинг
```bash
# Еженедельное сканирование
file_finder.exe -p C:\ -p D:\ --min-size 100MB -o weekly_scan_%date%.json --no-interactive

# Сравнение с прошлой неделей
file_finder.exe --load weekly_scan_previous.json
file_finder.exe --load weekly_scan_current.json
```

## 8. Советы и трюки

### Работа с большими коллекциями
Для дисков с миллионами файлов используйте большой минимальный размер:
```bash
file_finder.exe -p D:\ --min-size 1GB
```

### Быстрая проверка конкретной папки
```bash
file_finder.exe -p "C:\Program Files" --min-size 100MB --no-interactive
```

### Экспорт для анализа в Excel
```bash
# Сохраните в JSON, затем можно импортировать в Excel
file_finder.exe -o results.json
# results.json можно открыть в Excel или обработать скриптом
```

### Автоматизация через планировщик задач
Создайте batch файл для регулярного сканирования:
```batch
@echo off
set DATE=%date:~-4,4%%date:~-10,2%%date:~-7,2%
file_finder.exe -p C:\ --min-size 500MB -o "C:\Scans\scan_%DATE%.json" --no-interactive
```

## 9. Комбинации фильтров

### Медиа файлы определенного размера
```bash
# Видео между 500MB и 5GB (сначала сохраните все >500MB, потом фильтруйте)
file_finder.exe -e mp4,mkv --min-size 500MB
```

### Документы и архивы вместе
```bash
file_finder.exe -e pdf,docx,zip,rar --min-size 50MB
```

### Поиск возможных дубликатов по нескольким критериям
```bash
file_finder.exe -p C:\ -p D:\ --min-size 100MB --duplicates -o potential_duplicates.json
```

## 10. Устранение проблем

### Если сканирование слишком медленное
```bash
# Увеличьте минимальный размер
file_finder.exe --min-size 1GB

# Или используйте фильтр по расширениям
file_finder.exe -e mp4,mkv --min-size 100MB
```

### Если нужно просканировать конкретную папку, а не весь диск
```bash
file_finder.exe -p "C:\Users\YourName\Videos" --min-size 50MB
```

### Если файлов слишком много для просмотра
```bash
# Увеличьте размер страницы
file_finder.exe --page-size 100

# Или используйте no-interactive и работайте с JSON
file_finder.exe --no-interactive -o results.json
```
