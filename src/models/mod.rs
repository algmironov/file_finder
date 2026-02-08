use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub extension: String,
    pub modified: DateTime<Local>,
    pub hash: Option<String>,
}

impl FileInfo {
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

    pub fn file_name(&self) -> String {
        self.path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown")
            .to_string()
    }

    pub fn parent_dir(&self) -> String {
        self.path
            .parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResults {
    pub scan_start: DateTime<Local>,
    pub scan_end: DateTime<Local>,
    pub scanned_paths: Vec<String>,
    pub min_size_bytes: u64,
    pub extension_filter: Option<Vec<String>>,
    pub files: Vec<FileInfo>,
    pub total_scanned: u64,
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

    pub fn finalize(&mut self) {
        self.scan_end = Local::now();
        self.total_size = self.files.iter().map(|f| f.size).sum();
    }

    pub fn duration(&self) -> chrono::Duration {
        self.scan_end - self.scan_start
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileAction {
    OpenLocation,
    Delete,
    ShowDetails,
    Cancel,
}

impl FileAction {
    pub fn as_str(&self) -> &str {
        match self {
            FileAction::OpenLocation => "📂 Open folder",
            FileAction::Delete => "🗑️  Delete file",
            FileAction::ShowDetails => "ℹ️  Show details",
            FileAction::Cancel => "❌ Cancel",
        }
    }
}
