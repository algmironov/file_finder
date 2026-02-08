use crate::models::{FileAction, FileInfo};
use crate::utils;
use anyhow::Result;
use colored::*;
use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};

/// Interactive drive selection
pub fn select_drives(available_drives: Vec<String>) -> Result<Vec<String>> {
    if available_drives.is_empty() {
        anyhow::bail!("No available drives found");
    }

    println!("\n{}", "=== Select drives for scanning ===".bright_cyan().bold());
    println!("{}", "Use Space to select, Enter to confirm".dimmed());

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select drives")
        .items(&available_drives)
        .interact()?;

    if selections.is_empty() {
        anyhow::bail!("No drives selected");
    }

    let selected_drives: Vec<String> = selections
        .iter()
        .map(|&i| available_drives[i].clone())
        .collect();

    Ok(selected_drives)
}

/// Displays files with pagination
pub fn display_files_paginated(
    files: &[FileInfo],
    page_size: usize,
    extension_filter: Option<&str>,
) -> Result<()> {
    // Фильтруем файлы если указан фильтр
    let filtered_files: Vec<&FileInfo> = if let Some(ext) = extension_filter {
        files
            .iter()
            .filter(|f| f.extension.to_lowercase() == ext.to_lowercase())
            .collect()
    } else {
        files.iter().collect()
    };

    if filtered_files.is_empty() {
        println!("{}", "No files to display".yellow());
        return Ok(());
    }

    let total_pages = (filtered_files.len() + page_size - 1) / page_size;
    let mut current_page = 0;

    loop {
        // Clear screen (cross-platform)
        print!("\x1B[2J\x1B[1;1H");

        let start = current_page * page_size;
        let end = std::cmp::min(start + page_size, filtered_files.len());
        let page_files = &filtered_files[start..end];

        // Header
        println!("\n{}", "=".repeat(100).bright_cyan());
        println!(
            "{}",
            format!(
                "Found files (page {}/{}) | Total: {}",
                current_page + 1,
                total_pages,
                filtered_files.len()
            )
            .bright_cyan()
            .bold()
        );
        if let Some(ext) = extension_filter {
            println!("{}", format!("Filter: *.{}", ext).yellow());
        }
        println!("{}", "=".repeat(100).bright_cyan());

        // Table header
        println!(
            "{:<4} {:<4} {:<50} {:<15} {}",
            "#".bold(),
            "📁".bold(),
            "File name".bold(),
            "Size".bold(),
            "Path".bold()
        );
        println!("{}", "-".repeat(100).dimmed());

        for (idx, file) in page_files.iter().enumerate() {
            let icon = utils::get_file_icon(&file.extension);
            let file_name = file.file_name();
            let size = utils::format_size(file.size);
            let parent = file.parent_dir();

            // Обрезаем длинные имена
            let display_name = if file_name.len() > 47 {
                format!("{}...", &file_name[..44])
            } else {
                file_name
            };

            let display_path = if parent.len() > 50 {
                format!("...{}", &parent[parent.len() - 47..])
            } else {
                parent
            };

            println!(
                "{:<4} {:<4} {:<50} {:<15} {}",
                (start + idx + 1).to_string().bright_white(),
                icon,
                display_name.bright_white(),
                size.green(),
                display_path.dimmed()
            );
        }

        println!("\n{}", "=".repeat(100).bright_cyan());

        // Navigation menu
        let mut options = vec!["Select file for actions"];
        
        let prev_page_idx = if current_page > 0 {
            options.push("← Previous page");
            Some(options.len() - 1)
        } else {
            None
        };
        
        let next_page_idx = if current_page < total_pages - 1 {
            options.push("→ Next page");
            Some(options.len() - 1)
        } else {
            None
        };

        options.push("Apply extension filter");
        let filter_idx = options.len() - 1;
        
        options.push("Reset filter");
        let reset_idx = options.len() - 1;
        
        options.push("Exit");
        let exit_idx = options.len() - 1;

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose action")
            .items(&options)
            .default(0)
            .interact()?;

        // Handle selection by index
        if choice == 0 {
            // Select file for actions
            let file_options: Vec<String> = page_files
                .iter()
                .map(|f| {
                    format!(
                        "{} {} - {}",
                        utils::get_file_icon(&f.extension),
                        f.file_name(),
                        utils::format_size(f.size)
                    )
                })
                .collect();

            let file_choice = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select file")
                .items(&file_options)
                .interact()?;

            let selected_file = page_files[file_choice];
            file_action_menu(selected_file)?;
        } else if Some(choice) == prev_page_idx {
            current_page = current_page.saturating_sub(1);
        } else if Some(choice) == next_page_idx {
            current_page = std::cmp::min(current_page + 1, total_pages - 1);
        } else if choice == filter_idx {
            println!("{}", "This feature is available via command line arguments".yellow());
            std::thread::sleep(std::time::Duration::from_secs(2));
        } else if choice == reset_idx {
            return display_files_paginated(files, page_size, None);
        } else if choice == exit_idx {
            break;
        }
    }

    Ok(())
}

/// File action menu
fn file_action_menu(file: &FileInfo) -> Result<()> {
    println!("\n{}", "=== File Information ===".bright_cyan().bold());
    println!("Name: {}", file.file_name().bright_white());
    println!("Path: {}", file.path.display().to_string().dimmed());
    println!("Size: {}", utils::format_size(file.size).green());
    println!("Type: {}", file.extension.yellow());
    println!(
        "Modified: {}",
        file.modified.format("%Y-%m-%d %H:%M:%S").to_string().cyan()
    );

    let actions = vec![
        FileAction::OpenLocation.as_str(),
        FileAction::Delete.as_str(),
        FileAction::ShowDetails.as_str(),
        FileAction::Cancel.as_str(),
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose action")
        .items(&actions)
        .default(0)
        .interact()?;

    match choice {
        0 => {
            // Open location
            match utils::open_file_location(&file.path) {
                Ok(_) => println!("{}", "✓ Explorer opened".green()),
                Err(e) => println!("{}", format!("✗ Error: {}", e).red()),
            }
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
        1 => {
            // Delete file
            println!(
                "{}",
                format!("Are you sure you want to delete '{}'?", file.file_name())
                    .red()
                    .bold()
            );

            let confirm = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Confirm deletion")
                .default(false)
                .interact()?;

            if confirm {
                match utils::delete_file(&file.path) {
                    Ok(_) => println!("{}", "✓ File deleted".green()),
                    Err(e) => println!("{}", format!("✗ Delete error: {}", e).red()),
                }
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
        }
        2 => {
            // Show details
            println!("\n{}", "=== Detailed Information ===".bright_cyan().bold());
            println!("Full path: {}", file.path.display());
            println!("Size (bytes): {}", file.size);
            println!("Parent folder: {}", file.parent_dir());

            if let Some(ref hash) = file.hash {
                println!("SHA-256 hash: {}", hash);
            }

            println!("\nPress Enter to continue...");
            let _ = dialoguer::Input::<String>::new().allow_empty(true).interact();
        }
        _ => {}
    }

    Ok(())
}

/// Displays duplicate groups
pub fn display_duplicates(duplicate_groups: Vec<Vec<FileInfo>>) -> Result<()> {
    if duplicate_groups.is_empty() {
        println!("{}", "✓ No duplicates found!".green().bold());
        return Ok(());
    }

    println!(
        "\n{}",
        format!("Found {} duplicate groups", duplicate_groups.len())
            .yellow()
            .bold()
    );

    for (group_idx, group) in duplicate_groups.iter().enumerate() {
        println!("\n{}", format!("=== Group {} ===", group_idx + 1).bright_cyan().bold());
        println!(
            "Identical files: {} | Size each: {}",
            group.len(),
            utils::format_size(group[0].size).green()
        );
        println!(
            "Potential savings: {}",
            utils::format_size(group[0].size * (group.len() as u64 - 1))
                .red()
                .bold()
        );

        for (idx, file) in group.iter().enumerate() {
            println!(
                "  {}. {} - {}",
                idx + 1,
                file.file_name().bright_white(),
                file.parent_dir().dimmed()
            );
        }

        // Ask if user wants to delete duplicates
        if group.len() > 1 {
            let delete = dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Delete duplicates (keep first)?")
                .default(false)
                .interact()?;

            if delete {
                for file in group.iter().skip(1) {
                    match utils::delete_file(&file.path) {
                        Ok(_) => println!("  {} {}", "✓".green(), file.file_name()),
                        Err(e) => println!("  {} {} - {}", "✗".red(), file.file_name(), e),
                    }
                }
            }
        }
    }

    Ok(())
}
