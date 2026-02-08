use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Информация о найденном файле
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// Полный путь к файлу
    pub path: PathBuf,
    /// Размер файла в байтах
    pub size: u64,
    /// Расширение файла (например, "pdf", "mp4")
    pub extension: String,
    /// Время последнего изменения
    pub modified: DateTime<Local>,
    /// Хеш файла (вычисляется только для поиска дубликатов)
    pub hash: Option<String>,
}

impl FileInfo {
    /// Создает новый FileInfo из пути и метаданных
    pub fn new(path: PathBuf, size: u64, modified: DateTime<Local>) -> Self {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        Self {
            path,
            size,
            extension,
            modified,
            hash: None,
        }
    }

    /// Возвращает имя файла
    pub fn file_name(&self) -> String {
        self.path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown")
            .to_string()
    }

    /// Возвращает родительскую директорию
    pub fn parent_dir(&self) -> String {
        self.path
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .to_string()
    }
}

/// Результаты сканирования
#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResults {
    /// Время начала сканирования
    pub scan_start: DateTime<Local>,
    /// Время окончания сканирования
    pub scan_end: DateTime<Local>,
    /// Просканированные пути (диски/папки)
    pub scanned_paths: Vec<String>,
    /// Минимальный размер файла для поиска
    pub min_size_bytes: u64,
    /// Фильтр по расширениям (если был применен)
    pub extension_filter: Option<Vec<String>>,
    /// Найденные файлы
    pub files: Vec<FileInfo>,
    /// Общее количество просканированных файлов
    pub total_scanned: u64,
    /// Общий размер найденных файлов
    pub total_size: u64,
}

impl ScanResults {
    pub fn new(
        scanned_paths: Vec<String>,
        min_size_bytes: u64,
        extension_filter: Option<Vec<String>>,
    ) -> Self {
        Self {
            scan_start: Local::now(),
            scan_end: Local::now(),
            scanned_paths,
            min_size_bytes,
            extension_filter,
            files: Vec::new(),
            total_scanned: 0,
            total_size: 0,
        }
    }

    /// Завершает сканирование и обновляет статистику
    pub fn finalize(&mut self) {
        self.scan_end = Local::now();
        self.total_size = self.files.iter().map(|f| f.size).sum();
    }

    /// Возвращает продолжительность сканирования
    pub fn duration(&self) -> chrono::Duration {
        self.scan_end - self.scan_start
    }
}

/// Действия, которые можно выполнить с файлом
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileAction {
    /// Открыть расположение в проводнике
    OpenLocation,
    /// Удалить файл
    Delete,
    /// Показать детальную информацию
    ShowDetails,
    /// Отменить действие
    Cancel,
}

impl FileAction {
    pub fn as_str(&self) -> &str {
        match self {
            FileAction::OpenLocation => "📂 Открыть расположение",
            FileAction::Delete => "🗑️  Удалить файл",
            FileAction::ShowDetails => "ℹ️  Показать детали",
            FileAction::Cancel => "❌ Отмена",
        }
    }
}
