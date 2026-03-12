use crate::utils::to_pascal_case;
use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::Path;

pub fn update_icon_export(icon_name: &str, category: &str, folder_path: &str) -> io::Result<()> {
    fs::create_dir_all(folder_path)?;
    let file_path = Path::new(folder_path).join(format!("{}.ts", category.to_lowercase()));

    let file_path = file_path
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid UTF-8 path"))?;

    let pascal_name = to_pascal_case(icon_name);

    let mut existing_icons = read_existing_icons(&file_path);

    if existing_icons.insert(pascal_name.clone()) {
        write_icon_file(&file_path, &existing_icons)?;
        println!("Added {} to {}", pascal_name, file_path);
    } else {
        println!("Icon {} already exists in {}", pascal_name, file_path);
    }
    Ok(())
}

pub fn remove_icon_export(icon_name: &str, category: &str, folder_path: &str) -> io::Result<()> {
    let file_path = Path::new(folder_path).join(format!("{}.ts", category.to_lowercase()));
    let file_path = file_path
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid UTF-8 path"))?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::create_temp_dir;
    use std::collections::BTreeSet;

    // --- read_existing_icons ---

    #[test]
    fn test_read_existing_icons_parses_correctly() {
        let dir = create_temp_dir("test-export-", std::env::temp_dir());
        let path = dir.path().join("arrows.ts");
        fs::write(
            &path,
            "export {\n  ArrowUp,\n  ArrowDown,\n} from \"lucide-react\";\n",
        )
        .unwrap();

        let icons = read_existing_icons(path.to_str().unwrap());
        assert_eq!(
            icons,
            BTreeSet::from(["ArrowUp".into(), "ArrowDown".into()])
        );
    }

    #[test]
    fn test_read_existing_icons_deduplicates() {
        let dir = create_temp_dir("test-export-", std::env::temp_dir());
        let path = dir.path().join("arrows.ts");
        fs::write(&path, "export {\n  ArrowUp,\n  ArrowUp,\n} from \"lucide-react\";\n").unwrap();

        let icons = read_existing_icons(path.to_str().unwrap());
        assert_eq!(icons.len(), 1);
        assert!(icons.contains("ArrowUp"));
    }

    #[test]
    fn test_read_existing_icons_missing_file_returns_empty() {
        let icons = read_existing_icons("/nonexistent/path/file.ts");
        assert!(icons.is_empty());
    }

    #[test]
    fn test_read_existing_icons_empty_file_returns_empty() {
        let dir = create_temp_dir("test-export-", std::env::temp_dir());
        let path = dir.path().join("empty.ts");
        fs::write(&path, "").unwrap();

        let icons = read_existing_icons(path.to_str().unwrap());
        assert!(icons.is_empty());
    }

    // --- write_icon_file ---

    #[test]
    fn test_write_icon_file_creates_correct_content() {
        let dir = create_temp_dir("test-export-", std::env::temp_dir());
        let path = dir.path().join("icons.ts");
        let icons = BTreeSet::from(["ArrowDown".to_string(), "ArrowUp".to_string()]);

        write_icon_file(path.to_str().unwrap(), &icons).unwrap();

        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(
            content,
            "export {\n  ArrowDown,\n  ArrowUp,\n} from \"lucide-react\";\n"
        );
    }

    #[test]
    fn test_write_icon_file_empty_set_deletes_file() {
        let dir = create_temp_dir("test-export-", std::env::temp_dir());
        let path = dir.path().join("icons.ts");
        fs::write(&path, "export {\n  ArrowUp,\n} from \"lucide-react\";\n").unwrap();

        write_icon_file(path.to_str().unwrap(), &BTreeSet::new()).unwrap();

        assert!(!path.exists());
    }

    #[test]
    fn test_write_icon_file_empty_set_nonexistent_file_is_ok() {
        let dir = create_temp_dir("test-export-", std::env::temp_dir());
        let path = dir.path().join("nonexistent.ts");

        // Should not error even if file doesn't exist
        write_icon_file(path.to_str().unwrap(), &BTreeSet::new()).unwrap();
    }

    // --- update_icon_export ---

    #[test]
    fn test_update_icon_export_adds_new_icon() {
        let dir = create_temp_dir("test-export-", std::env::temp_dir());
        update_icon_export("arrow-up", "arrows", dir.path().to_str().unwrap()).unwrap();

        let content = fs::read_to_string(dir.path().join("arrows.ts")).unwrap();
        assert!(content.contains("ArrowUp"));
        assert!(content.contains("lucide-react"));
    }

    #[test]
    fn test_update_icon_export_duplicate_is_idempotent() {
        let dir = create_temp_dir("test-export-", std::env::temp_dir());
        let dir_str = dir.path().to_str().unwrap();

        update_icon_export("arrow-up", "arrows", dir_str).unwrap();
        update_icon_export("arrow-up", "arrows", dir_str).unwrap();

        let content = fs::read_to_string(dir.path().join("arrows.ts")).unwrap();
        assert_eq!(content.matches("ArrowUp").count(), 1);
    }

    #[test]
    fn test_update_icon_export_creates_directory_if_missing() {
        let dir = create_temp_dir("test-export-", std::env::temp_dir());
        let nested = dir.path().join("sub").join("dir");

        update_icon_export("arrow-up", "arrows", nested.to_str().unwrap()).unwrap();

        assert!(nested.join("arrows.ts").exists());
    }

    #[test]
    fn test_update_icon_export_icons_are_sorted() {
        let dir = create_temp_dir("test-export-", std::env::temp_dir());
        let dir_str = dir.path().to_str().unwrap();

        update_icon_export("arrow-up", "arrows", dir_str).unwrap();
        update_icon_export("arrow-down", "arrows", dir_str).unwrap();

        let content = fs::read_to_string(dir.path().join("arrows.ts")).unwrap();
        let down_pos = content.find("ArrowDown").unwrap();
        let up_pos = content.find("ArrowUp").unwrap();
        assert!(down_pos < up_pos, "icons should be sorted alphabetically");
    }

    // --- remove_icon_export ---

    #[test]
    fn test_remove_icon_export_removes_existing_icon() {
        let dir = create_temp_dir("test-export-", std::env::temp_dir());
        let path = dir.path().join("arrows.ts");
        fs::write(
            &path,
            "export {\n  ArrowDown,\n  ArrowUp,\n} from \"lucide-react\";\n",
        )
        .unwrap();

        remove_icon_export("arrow-up", "arrows", dir.path().to_str().unwrap()).unwrap();

        let content = fs::read_to_string(&path).unwrap();
        assert!(!content.contains("ArrowUp"));
        assert!(content.contains("ArrowDown"));
    }

    #[test]
    fn test_remove_icon_export_last_icon_deletes_file() {
        let dir = create_temp_dir("test-export-", std::env::temp_dir());
        let path = dir.path().join("arrows.ts");
        fs::write(&path, "export {\n  ArrowUp,\n} from \"lucide-react\";\n").unwrap();

        remove_icon_export("arrow-up", "arrows", dir.path().to_str().unwrap()).unwrap();

        assert!(!path.exists());
    }

    #[test]
    fn test_remove_icon_export_nonexistent_icon_is_ok() {
        let dir = create_temp_dir("test-export-", std::env::temp_dir());
        let path = dir.path().join("arrows.ts");
        fs::write(&path, "export {\n  ArrowUp,\n} from \"lucide-react\";\n").unwrap();

        remove_icon_export("arrow-down", "arrows", dir.path().to_str().unwrap()).unwrap();

        // File should be untouched
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("ArrowUp"));
    }
}
