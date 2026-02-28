use crate::utils::to_pascal_case;
use std::collections::BTreeSet;
use std::fs;
use std::io;

pub fn update_icon_export(icon_name: &str, category: &str, folder_path: &str) -> io::Result<()> {
    fs::create_dir_all(folder_path)?;
    let file_path = format!("{}/{}.ts", folder_path, category.to_lowercase());
    let pascal_name = to_pascal_case(icon_name);

    let mut icons = read_existing_icons(&file_path);
    if icons.insert(pascal_name.clone()) {
        write_icon_file(&file_path, &icons)?;
        println!("Added {} to {}", pascal_name, file_path);
    } else {
        println!("Icon {} already exists in {}", pascal_name, file_path);
    }
    Ok(())
}

pub fn remove_icon_export(icon_name: &str, category: &str, folder_path: &str) -> io::Result<()> {
    let file_path = format!("{}/{}.ts", folder_path, category.to_lowercase());
    let pascal_name = to_pascal_case(icon_name);

    let mut icons = read_existing_icons(&file_path);
    if icons.remove(&pascal_name) {
        write_icon_file(&file_path, &icons)?;
        println!("Removed {} from {}", pascal_name, file_path);
    } else {
        println!("Icon {} not found in {}", pascal_name, file_path);
    }
    Ok(())
}

fn read_existing_icons(path: &str) -> BTreeSet<String> {
    let content = fs::read_to_string(path).unwrap_or_default();
    let mut icons = BTreeSet::new();
    if let (Some(start), Some(end)) = (content.find('{'), content.rfind('}')) {
        for line in content[start + 1..end].split(',') {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                icons.insert(trimmed.to_string());
            }
        }
    }
    icons
}

fn write_icon_file(path: &str, icons: &BTreeSet<String>) -> io::Result<()> {
    if icons.is_empty() {
        // Optional: delete file if empty, or just leave empty export
        return fs::remove_file(path).or(Ok(()));
    }
    let mut output = String::from("export {\n");
    for icon in icons {
        output.push_str(&format!("  {},\n", icon));
    }
    output.push_str("} from \"lucide-react\";\n");
    fs::write(path, output)
}
