mod export_manager;
mod models;
mod utils;

use crate::export_manager::{remove_icon_export, update_icon_export};
use crate::models::LucideIcon;
use reqwest::blocking::get;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage();
        return Ok(());
    }

    let command = &args[1];
    let icon_name = &args[2];
    let folder_path = args.get(3).map(|s| s.as_str()).unwrap_or("icons");

    // We fetch the icon data for both commands to identify the correct category file
    let url = format!(
        "https://raw.githubusercontent.com/lucide-icons/lucide/main/icons/{icon_name}.json"
    );
    let response = get(url)?;

    if !response.status().is_success() {
        println!("Icon '{}' not found in Lucide repository.", icon_name);
        return Ok(());
    }

    let icon: LucideIcon = response.json()?;
    let category = match icon.categories.first() {
        Some(cat) => cat,
        None => {
            println!("No category found for icon '{}'.", icon_name);
            return Ok(());
        }
    };

    match command.as_str() {
        "add" => update_icon_export(icon_name, category, folder_path)?,
        "remove" => remove_icon_export(icon_name, category, folder_path)?,
        _ => print_usage(),
    }

    Ok(())
}

fn print_usage() {
    println!("Usage:");
    println!("  icons-manager add <icon-name> [folder-path]");
    println!("  icons-manager remove <icon-name> [folder-path]");
}
