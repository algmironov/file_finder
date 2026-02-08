use file_finder::scanner::{ScanConfig, scan_files};
use file_finder::utils;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;

/// Создает тестовую структуру файлов
fn create_test_files(dir: &PathBuf) -> std::io::Result<()> {
    // Создаем несколько тестовых файлов разного размера
    
    // Большой файл (150KB)
    let mut large_file = File::create(dir.join("large_video.mp4"))?;
    large_file.write_all(&vec![0u8; 150 * 1024])?;
    
    // Средний файл (50KB)
    let mut medium_file = File::create(dir.join("medium_doc.pdf"))?;
    medium_file.write_all(&vec![1u8; 50 * 1024])?;
    
    // Маленький файл (5KB)
    let mut small_file = File::create(dir.join("small_image.jpg"))?;
    small_file.write_all(&vec![2u8; 5 * 1024])?;
    
    // Файл без расширения
    let mut no_ext_file = File::create(dir.join("README"))?;
    no_ext_file.write_all(b"This is a readme file")?;
    
    // Создаем поддиректорию с файлами
    let subdir = dir.join("subdir");
    fs::create_dir(&subdir)?;
    
    let mut nested_file = File::create(subdir.join("nested_archive.zip"))?;
    nested_file.write_all(&vec![3u8; 100 * 1024])?;
    
    Ok(())
}

#[test]
fn test_scan_basic() {
    let temp_dir = TempDir::new().unwrap();
    let test_path = temp_dir.path().to_path_buf();
    
    create_test_files(&test_path).unwrap();
    
    let config = ScanConfig {
        paths: vec![test_path.to_str().unwrap().to_string()],
        min_size: 10 * 1024, // 10KB минимум
        extensions: None,
        show_progress: false,
    };
    
    let results = scan_files(config).unwrap();
    
    // Должны найти файлы больше 10KB: large_video, medium_doc, nested_archive
    assert!(results.files.len() >= 3, "Expected at least 3 files, found {}", results.files.len());
    
    // Проверяем, что самый большой файл - large_video.mp4
    assert!(results.files[0].extension == "mp4");
    assert!(results.files[0].size >= 150 * 1024);
}

#[test]
fn test_scan_with_extension_filter() {
    let temp_dir = TempDir::new().unwrap();
    let test_path = temp_dir.path().to_path_buf();
    
    create_test_files(&test_path).unwrap();
    
    let config = ScanConfig {
        paths: vec![test_path.to_str().unwrap().to_string()],
        min_size: 1024, // 1KB минимум
        extensions: Some(vec!["mp4".to_string(), "zip".to_string()]),
        show_progress: false,
    };
    
    let results = scan_files(config).unwrap();
    
    // Должны найти только mp4 и zip файлы
    assert_eq!(results.files.len(), 2, "Expected 2 files with mp4/zip extensions");
    
    // Проверяем расширения
    for file in &results.files {
        assert!(file.extension == "mp4" || file.extension == "zip");
    }
}

#[test]
fn test_scan_min_size_filter() {
    let temp_dir = TempDir::new().unwrap();
    let test_path = temp_dir.path().to_path_buf();
    
    create_test_files(&test_path).unwrap();
    
    let config = ScanConfig {
        paths: vec![test_path.to_str().unwrap().to_string()],
        min_size: 100 * 1024, // 100KB минимум
        extensions: None,
        show_progress: false,
    };
    
    let results = scan_files(config).unwrap();
    
    // Должны найти только файлы >= 100KB: large_video и nested_archive
    assert!(results.files.len() >= 2);
    
    // Проверяем, что все файлы больше минимального размера
    for file in &results.files {
        assert!(file.size >= 100 * 1024, "File {} is smaller than min_size", file.path.display());
    }
}

#[test]
fn test_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let test_path = temp_dir.path().to_path_buf();
    
    let config = ScanConfig {
        paths: vec![test_path.to_str().unwrap().to_string()],
        min_size: 0,
        extensions: None,
        show_progress: false,
    };
    
    let results = scan_files(config).unwrap();
    
    // Пустая директория должна вернуть 0 файлов
    assert_eq!(results.files.len(), 0);
    assert_eq!(results.total_scanned, 0);
}

#[cfg(test)]
mod utils_tests {
    use super::*;
    
    #[test]
    fn test_parse_size_string() {
        assert_eq!(utils::parse_size_string("100MB").unwrap(), 100 * 1024 * 1024);
        assert_eq!(utils::parse_size_string("1GB").unwrap(), 1024 * 1024 * 1024);
        assert_eq!(utils::parse_size_string("500KB").unwrap(), 500 * 1024);
        assert_eq!(utils::parse_size_string("1024").unwrap(), 1024);
        assert_eq!(utils::parse_size_string("2GiB").unwrap(), 2 * 1024 * 1024 * 1024);
    }
    
    #[test]
    fn test_format_size() {
        assert_eq!(utils::format_size(1024), "1.0 KB");
        assert_eq!(utils::format_size(1024 * 1024), "1.0 MB");
        assert_eq!(utils::format_size(1536), "1.5 KB");
    }
    
    #[test]
    fn test_get_file_icon() {
        assert_eq!(utils::get_file_icon("mp4"), "🎬");
        assert_eq!(utils::get_file_icon("MP3"), "🎵");
        assert_eq!(utils::get_file_icon("pdf"), "📕");
        assert_eq!(utils::get_file_icon("ZIP"), "📦");
        assert_eq!(utils::get_file_icon("unknown"), "📁");
    }
}
