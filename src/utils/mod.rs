use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Получает список доступных дисков в системе
#[cfg(windows)]
pub fn get_available_drives() -> Result<Vec<String>> {
    use windows::Win32::Storage::FileSystem::GetLogicalDrives;

    unsafe {
        let drives_mask = GetLogicalDrives();
        let mut drives = Vec::new();

        for i in 0..26 {
            if drives_mask & (1 << i) != 0 {
                let drive_letter = (b'A' + i) as char;
                drives.push(format!("{}:\\", drive_letter));
            }
        }

        Ok(drives)
    }
}

/// Для Unix-систем просто возвращаем корень
#[cfg(not(windows))]
pub fn get_available_drives() -> Result<Vec<String>> {
    Ok(vec!["/".to_string()])
}

/// Форматирует размер файла в читаемый вид
pub fn format_size(bytes: u64) -> String {
    bytesize::ByteSize::b(bytes).to_string_as(true)
}

/// Вычисляет SHA-256 хеш файла для поиска дубликатов
/// Читает файл блоками по 8KB для эффективности
pub fn calculate_file_hash<P: AsRef<Path>>(path: P) -> Result<String> {
    let file = File::open(path.as_ref())
        .with_context(|| format!("Не удалось открыть файл: {:?}", path.as_ref()))?;

    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192]; // 8KB буфер

    loop {
        let bytes_read = reader
            .read(&mut buffer)
            .context("Ошибка чтения файла")?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// Открывает папку в проводнике файлов
pub fn open_file_location<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    
    // Получаем родительскую директорию
    let dir = path
        .parent()
        .context("Не удалось получить родительскую директорию")?;

    opener::open(dir).context("Не удалось открыть проводник")?;

    Ok(())
}

/// Удаляет файл
pub fn delete_file<P: AsRef<Path>>(path: P) -> Result<()> {
    std::fs::remove_file(path.as_ref())
        .with_context(|| format!("Не удалось удалить файл: {:?}", path.as_ref()))?;
    Ok(())
}

/// Получает иконку для типа файла (в виде эмодзи)
pub fn get_file_icon(extension: &str) -> &str {
    match extension.to_lowercase().as_str() {
        // Видео
        "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" => "🎬",
        
        // Аудио
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" | "m4a" => "🎵",
        
        // Изображения
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" | "ico" => "🖼️",
        
        // Документы
        "pdf" => "📕",
        "doc" | "docx" => "📘",
        "xls" | "xlsx" => "📊",
        "ppt" | "pptx" => "📽️",
        "txt" | "md" => "📄",
        
        // Архивы
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" => "📦",
        
        // Исполняемые файлы
        "exe" | "msi" | "app" => "⚙️",
        
        // Код
        "rs" | "py" | "js" | "ts" | "java" | "cpp" | "c" | "go" => "💻",
        
        // ISO образы
        "iso" | "img" => "💿",
        
        // По умолчанию
        _ => "📁",
    }
}

/// Парсит строку размера (например, "100MB") в байты
pub fn parse_size_string(size_str: &str) -> Result<u64> {
    let size_str = size_str.trim().to_uppercase();
    
    let (num_str, multiplier) = if size_str.ends_with("GB") || size_str.ends_with("GIB") {
        (size_str.trim_end_matches("GB").trim_end_matches("GIB"), 1024u64.pow(3))
    } else if size_str.ends_with("MB") || size_str.ends_with("MIB") {
        (size_str.trim_end_matches("MB").trim_end_matches("MIB"), 1024u64.pow(2))
    } else if size_str.ends_with("KB") || size_str.ends_with("KIB") {
        (size_str.trim_end_matches("KB").trim_end_matches("KIB"), 1024)
    } else if size_str.ends_with("B") {
        (size_str.trim_end_matches("B"), 1)
    } else {
        (size_str.as_str(), 1)
    };
    
    let number: u64 = num_str.trim().parse()
        .context("Неверный формат размера. Используйте формат: 100MB, 1GB и т.д.")?;
    
    Ok(number * multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size_string() {
        assert_eq!(parse_size_string("100MB").unwrap(), 100 * 1024 * 1024);
        assert_eq!(parse_size_string("1GB").unwrap(), 1024 * 1024 * 1024);
        assert_eq!(parse_size_string("500KB").unwrap(), 500 * 1024);
        assert_eq!(parse_size_string("1024").unwrap(), 1024);
    }

    #[test]
    fn test_get_file_icon() {
        assert_eq!(get_file_icon("mp4"), "🎬");
        assert_eq!(get_file_icon("MP3"), "🎵");
        assert_eq!(get_file_icon("pdf"), "📕");
        assert_eq!(get_file_icon("unknown"), "📁");
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1024 * 1024), "1.0 MB");
    }
}
