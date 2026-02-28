use reqwest::blocking::get;
use serde::Deserialize;
use std::error::Error;
use std::io::{self, Write};

#[derive(Deserialize, Debug)]
struct LucideIcon {
    tags: Vec<String>,
    categories: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    print!("Enter icon name (e.g., a-arrow-up): ");
    io::stdout().flush()?;

    let mut icon_name = String::new();
    io::stdin().read_line(&mut icon_name)?;
    let icon_name = icon_name.trim();

    let url = format!(
        "https://raw.githubusercontent.com/lucide-icons/lucide/main/icons/{icon_name}.json"
    );

    let response = get(url)?;

    if response.status().is_success() {
        let icon: LucideIcon = response.json()?;

        println!("Icon name: {icon_name}");
        println!("Icon tags: {:?}", icon.tags);
        println!("Icon categories: {:?}", icon.categories);
    } else if response.status().as_u16() == 404 {
        println!("Icon '{}' does not exist in Lucide.", icon_name);
    } else {
        println!("Failed to fetch icon. HTTP status: {}", response.status());
    }

    Ok(())
}
