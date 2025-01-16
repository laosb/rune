use std::path::PathBuf;
use std::sync::Arc;

use colored::*;
use tokio::sync::RwLock;
use unicode_width::UnicodeWidthStr;

use crate::cli::Command;
use crate::fs::VirtualFS;

pub async fn execute(
    command: Command,
    fs: &Arc<RwLock<VirtualFS>>,
) -> Result<bool, Box<dyn std::error::Error>> {
    match command {
        Command::Ls { long } => {
            let fs = fs.read().await;
            match fs.list_current_dir().await {
                Ok(entries) => {
                    if long {
                        // Detailed mode (ls -l)
                        for entry in entries {
                            let entry_type = if entry.is_directory { "DIR" } else { "FILE" };
                            let id_str =
                                entry.id.map(|id| format!(" [{}]", id)).unwrap_or_default();
                            println!("{:<4} {}{}", entry_type, entry.name, id_str);
                        }
                    } else {
                        // Simple mode (ls)
                        let mut entries = entries;
                        entries.sort_by(|a, b| a.name.cmp(&b.name));

                        // Calculate terminal width
                        let term_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);

                        let column_spacing = 2;

                        // Calculate the width of the longest entry
                        let max_name_width =
                            entries.iter().map(|e| e.name.width()).max().unwrap_or(0);

                        let column_width = max_name_width + column_spacing;

                        // Calculate the number of entries per line
                        let cols = std::cmp::max(1, term_width / column_width);

                        // Prepare for display
                        let mut current_col = 0;
                        for entry in entries {
                            let name = if entry.is_directory {
                                entry.name.blue().bold().to_string()
                            } else {
                                entry.name.clone()
                            };

                            let display_width = entry.name.width();

                            print!("{}", name);
                            for _ in 0..(column_width - display_width) {
                                print!(" ");
                            }

                            current_col += 1;
                            if current_col >= cols {
                                println!();
                                current_col = 0;
                            }
                        }

                        // Print a newline if the last line is incomplete
                        if current_col != 0 {
                            println!();
                        }
                    }
                }
                Err(e) => eprintln!("Error listing directory: {}", e),
            }
            Ok(true)
        }
        Command::Pwd => {
            let fs = fs.read().await;
            println!("Current directory: {}", fs.current_dir().to_string_lossy());
            Ok(true)
        }
        Command::Cd { path } => {
            let mut fs = fs.write().await;
            let new_path = match path.as_str() {
                "." => fs.current_path.clone(),
                ".." => {
                    if fs.current_path != std::path::Path::new("/") {
                        let mut new_path = fs.current_path.clone();
                        new_path.pop();
                        new_path
                    } else {
                        fs.current_path.clone()
                    }
                }
                "/" => PathBuf::from("/"),
                path => fs.current_path.join(path),
            };

            match fs.validate_path(&new_path).await {
                Ok(true) => {
                    fs.current_path = new_path;
                }
                Ok(false) => {
                    println!("Directory not found: {}", path);
                }
                Err(e) => {
                    println!("Error validating path: {}", e);
                }
            }
            Ok(true)
        }
        Command::Quit => Ok(false),
    }
}
