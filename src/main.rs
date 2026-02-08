mod models;
mod scanner;
mod ui;
mod utils;

use anyhow::Result;
use clap::Parser;
use colored::*;
use std::path::PathBuf;

/// Утилита для быстрого поиска больших файлов на Windows
#[derive(Parser, Debug)]
#[command(name = "FileFinder")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "Быстрый поиск больших файлов с интерактивными возможностями", long_about = None)]
struct Args {
    /// Минимальный размер файла (например: 100MB, 1GB)
    /// По умолчанию: 100MB
    #[arg(short, long, default_value = "100MB")]
    min_size: String,

    /// Расширения файлов для фильтрации (например: mp4,mkv,avi)
    /// Можно указать несколько через запятую
    #[arg(short, long, value_delimiter = ',')]
    extensions: Option<Vec<String>>,

    /// Пути для сканирования (если не указаны, будет интерактивный выбор дисков)
    #[arg(short, long)]
    paths: Option<Vec<String>>,

    /// Сохранить результаты в JSON файл
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Загрузить ранее сохраненные результаты
    #[arg(short, long)]
    load: Option<PathBuf>,

    /// Найти дубликаты файлов (по SHA-256 хешу)
    #[arg(short, long)]
    duplicates: bool,

    /// Количество файлов на странице при просмотре
    #[arg(long, default_value = "20")]
    page_size: usize,

    /// Не показывать интерактивное меню после сканирования
    #[arg(long)]
    no_interactive: bool,
}

fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Print banner
    print_banner();

    // If loading results from file
    if let Some(load_path) = args.load {
        return load_and_display_results(load_path, args.page_size, args.no_interactive);
    }

    // Parse minimum size
    let min_size_bytes = utils::parse_size_string(&args.min_size)?;

    println!(
        "{}",
        format!("Minimum file size: {}", utils::format_size(min_size_bytes))
            .bright_green()
    );

    // Get paths to scan
    let paths = if let Some(p) = args.paths {
        p
    } else {
        // Interactive drive selection
        let available_drives = utils::get_available_drives()?;
        ui::select_drives(available_drives)?
    };

    println!("{}", format!("Scanning paths: {:?}", paths).bright_blue());

    if let Some(ref exts) = args.extensions {
        println!(
            "{}",
            format!("Extension filter: {}", exts.join(", ")).yellow()
        );
    }

    // Create scan configuration
    let mut config = scanner::ScanConfig::new(paths, min_size_bytes);

    if let Some(extensions) = args.extensions.clone() {
        config = config.with_extensions(extensions);
    }

    // Start scanning
    println!("\n{}", "Starting scan...".bright_cyan().bold());
    let mut results = scanner::scan_files(config)?;

    // Print statistics
    print_statistics(&results);

    // Find duplicates if flag is set
    if args.duplicates && !results.files.is_empty() {
        println!(
            "\n{}",
            "Finding duplicates (calculating hashes)...".bright_cyan().bold()
        );
        let duplicate_groups = scanner::find_duplicates(&mut results.files, true)?;
        ui::display_duplicates(duplicate_groups)?;
    }

    // Save results
    if let Some(output_path) = args.output {
        save_results(&results, &output_path)?;
    }

    // Interactive mode
    if !args.no_interactive && !results.files.is_empty() {
        println!("\n{}", "=== Interactive Mode ===".bright_cyan().bold());
        ui::display_files_paginated(&results.files, args.page_size, None)?;
    }

    println!("\n{}", "Done!".bright_green().bold());

    Ok(())
}

/// Prints a nice banner
fn print_banner() {
    println!("{}", "╔════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║      🔍 FileFinder v1.0 🔍            ║".bright_cyan().bold());
    println!("{}", "║    Fast file finder utility            ║".bright_cyan());
    println!("{}", "╚════════════════════════════════════════╝".bright_cyan());
    println!();
}

/// Prints scan statistics
fn print_statistics(results: &models::ScanResults) {
    println!("\n{}", "=== Scan Statistics ===".bright_cyan().bold());
    println!(
        "⏱️  Scan time: {} sec",
        results.duration().num_seconds()
    );
    println!("📊 Total scanned: {} files", results.total_scanned);
    println!("✅ Found matching: {} files", results.files.len());
    println!(
        "💾 Total size: {}",
        utils::format_size(results.total_size).green().bold()
    );

    if !results.files.is_empty() {
        println!(
            "📈 Largest file: {} ({})",
            results.files[0].file_name().bright_white(),
            utils::format_size(results.files[0].size).red().bold()
        );
    }

    println!("{}", "=".repeat(50).bright_cyan());
}

/// Saves results to JSON file
fn save_results(results: &models::ScanResults, path: &PathBuf) -> Result<()> {
    let json = serde_json::to_string_pretty(results)?;
    std::fs::write(path, json)?;
    println!(
        "{}",
        format!("✓ Results saved to: {}", path.display())
            .green()
            .bold()
    );
    Ok(())
}

/// Loads and displays previously saved results
fn load_and_display_results(
    path: PathBuf,
    page_size: usize,
    no_interactive: bool,
) -> Result<()> {
    println!(
        "{}",
        format!("Loading results from: {}", path.display())
            .bright_blue()
            .bold()
    );

    let json = std::fs::read_to_string(&path)?;
    let results: models::ScanResults = serde_json::from_str(&json)?;

    print_statistics(&results);

    if !no_interactive && !results.files.is_empty() {
        ui::display_files_paginated(&results.files, page_size, None)?;
    }

    Ok(())
}
