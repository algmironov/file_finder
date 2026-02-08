use crate::models::{FileInfo, ScanResults};
use anyhow::Result;
use chrono::{DateTime, Local};
use indicatif::{ProgressBar, ProgressStyle};
use jwalk::WalkDir;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct ScanConfig {
    pub paths: Vec<String>,
    pub min_size: u64,
    pub extensions: Option<Vec<String>>,
    pub show_progress: bool,
}

impl ScanConfig {
    pub fn new(paths: Vec<String>, min_size: u64) -> Self {
        Self {
            paths,
            min_size,
            extensions: None,
            show_progress: true,
        }
    }

    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = Some(extensions);
        self
    }
}

pub fn scan_files(config: ScanConfig) -> Result<ScanResults> {
    let mut results = ScanResults::new(
        config.paths.clone(),
        config.min_size,
        config.extensions.clone(),
    );

    // Создаем прогресс-бар
    let progress = if config.show_progress {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    let scanned_count = Arc::new(AtomicU64::new(0));
    let found_count = Arc::new(AtomicU64::new(0));

    let mut all_files = Vec::new();

    for path in &config.paths {
        if let Some(ref pb) = progress {
            pb.set_message(format!("Scanning: {}", path));
        }

        let walker = WalkDir::new(path)
            .parallelism(jwalk::Parallelism::RayonNewPool(8))
            .skip_hidden(false);

        for entry_result in walker {
            
            match entry_result {
                Ok(entry) => {
                    let count = scanned_count.fetch_add(1, Ordering::Relaxed);

                    if count % 1000 == 0 {
                        if let Some(ref pb) = progress {
                            pb.set_message(format!(
                                "Scanning: {} | Files: {} | Found: {}",
                                path,
                                count,
                                found_count.load(Ordering::Relaxed)
                            ));
                        }
                    }

                    if !entry.file_type().is_file() {
                        continue;
                    }

                    let metadata = match entry.metadata() {
                        Ok(m) => m,
                        Err(_) => continue,
                    };

                    let size = metadata.len();

                    if size < config.min_size {
                        continue;
                    }

                    let path_buf = entry.path();

                    if let Some(ref exts) = config.extensions {
                        let ext = path_buf
                            .extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("")
                            .to_lowercase();

                        if !exts.iter().any(|e| e.to_lowercase() == ext) {
                            continue;
                        }
                    }

                    let modified = metadata
                        .modified()
                        .ok()
                        .and_then(|t| system_time_to_datetime(t))
                        .unwrap_or_else(|| Local::now());

                    let file_info = FileInfo::new(path_buf, size, modified);
                    all_files.push(file_info);
                    found_count.fetch_add(1, Ordering::Relaxed);
                }
                Err(_) => {
                    continue;
                }
            }
        }
    }

    if let Some(pb) = progress {
        pb.finish_with_message(format!(
            "✓ Scan complete! Scanned: {} | Found: {}",
            scanned_count.load(Ordering::Relaxed),
            found_count.load(Ordering::Relaxed)
        ));
    }

    all_files.sort_by(|a, b| b.size.cmp(&a.size));

    results.files = all_files;
    results.total_scanned = scanned_count.load(Ordering::Relaxed);
    results.finalize();

    Ok(results)
}

fn system_time_to_datetime(system_time: SystemTime) -> Option<DateTime<Local>> {
    let duration_since_epoch = system_time.duration_since(SystemTime::UNIX_EPOCH).ok()?;
    DateTime::from_timestamp(duration_since_epoch.as_secs() as i64, 0)
        .map(|dt| dt.with_timezone(&Local))
}

pub fn find_duplicates(files: &mut [FileInfo], show_progress: bool) -> Result<Vec<Vec<FileInfo>>> {
    use rayon::prelude::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    let progress = if show_progress {
        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("=>-")
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        Some(Arc::new(Mutex::new(pb)))
    } else {
        None
    };

    files.par_iter_mut().for_each(|file| {
        if let Ok(hash) = crate::utils::calculate_file_hash(&file.path) {
            file.hash = Some(hash);
        }
        if let Some(ref pb) = progress {
            if let Ok(pb) = pb.lock() {
                let file_name = file.file_name();
                let display_name = if file_name.len() > 40 {
                    format!("{}...", &file_name[..37])
                } else {
                    file_name
                };
                pb.set_message(format!("Hashing: {}", display_name));
                pb.inc(1);
            }
        }
    });

    if let Some(pb) = progress {
        if let Ok(pb) = pb.lock() {
            pb.finish_with_message("✓ Hashing complete");
        }
    }

    let mut hash_map: HashMap<String, Vec<FileInfo>> = HashMap::new();

    for file in files.iter() {
        if let Some(ref hash) = file.hash {
            hash_map
                .entry(hash.clone())
                .or_insert_with(Vec::new)
                .push(file.clone());
        }
    }

    let duplicates: Vec<Vec<FileInfo>> = hash_map
        .into_iter()
        .map(|(_, files)| files)
        .filter(|group| group.len() > 1)
        .collect();

    Ok(duplicates)
}
