use icons_master::utils::create_temp_dir;
use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_icon() {
        let temp_dir = create_temp_dir("test-icons-", PathBuf::from("tests"));
        let dir = temp_dir.path().to_str().unwrap();

        // Add arrow-up
        let out = run_cli(&["add", "arrow-up", dir]);
        assert!(out.status.success());
        assert!(stdout_to_string(&out).contains("Added"));

        let file_path = Path::new(dir).join("arrows.ts");
        assert_eq!(file_contains(&file_path, "ArrowUp"), 1);
    }

    #[test]
    fn test_add_duplicate_icon() {
        let temp_dir = create_temp_dir("test-icons-", PathBuf::from("tests"));
        let dir = temp_dir.path().to_str().unwrap();

        // Add arrow-up
        let out = run_cli(&["add", "arrow-up", dir]);
        assert!(out.status.success());
        assert!(stdout_to_string(&out).contains("Added"));

        let file_path = Path::new(dir).join("arrows.ts");
        assert_eq!(file_contains(&file_path, "ArrowUp"), 1);

        // Duplicate add
        let out = run_cli(&["add", "arrow-up", dir]);
        assert!(out.status.success());
        assert!(stdout_to_string(&out).contains("already exists"));

        let file_path = Path::new(dir).join("arrows.ts");
        assert_eq!(file_contains(&file_path, "ArrowUp"), 1);
    }

    #[test]
    fn test_add_multiple_icons() {
        let temp_dir = create_temp_dir("test-icons-", PathBuf::from("tests"));
        let dir = temp_dir.path();
        let dir_str = dir.to_str().unwrap();

        // Add one icon
        let out = run_cli(&["add", "arrow-up", dir_str]);
        assert!(out.status.success());
        assert!(stdout_to_string(&out).contains("Added"));

        let file_path = Path::new(dir).join("arrows.ts");
        assert_eq!(file_contains(&file_path, "ArrowUp"), 1);
        assert_eq!(file_contains(&file_path, "ArrowDown"), 0);

        // Add another icon
        let out = run_cli(&["add", "arrow-down", dir_str]);
        assert!(out.status.success());
        assert!(stdout_to_string(&out).contains("Added"));

        assert_eq!(file_contains(&file_path, "ArrowUp"), 1);
        assert_eq!(file_contains(&file_path, "ArrowDown"), 1);
    }

    #[test]
    fn test_remove_icon() {
        let temp_dir = create_temp_dir("test-icons-", PathBuf::from("tests"));
        let dir = temp_dir.path();

        std::fs::write(
            dir.join("arrows.ts"),
            r#"export { ArrowUp, ArrowDown } from './icons/ArrowUp';"#,
        )
        .unwrap();

        let out = run_cli(&["remove", "arrow-up", dir.to_str().unwrap()]);
        assert!(out.status.success());
        assert!(stdout_to_string(&out).contains("Removed"));

        assert_eq!(file_contains(&dir.join("arrows.ts"), "ArrowUp"), 0);
        assert_eq!(file_contains(&dir.join("arrows.ts"), "ArrowDown"), 1);
    }

    #[test]
    fn test_remove_multiple_icons() {
        let temp_dir = create_temp_dir("test-icons-", PathBuf::from("tests"));
        let dir = temp_dir.path();

        std::fs::write(
            dir.join("arrows.ts"),
            r#"export { ArrowUp, ArrowDown } from './icons/ArrowUp';"#,
        )
        .unwrap();

        // Remove one icon
        let out = run_cli(&["remove", "arrow-up", dir.to_str().unwrap()]);
        assert!(out.status.success());
        assert!(stdout_to_string(&out).contains("Removed"));

        let file_path = dir.join("arrows.ts");

        assert_eq!(file_contains(&file_path, "ArrowUp"), 0);
        assert_eq!(file_contains(&file_path, "ArrowDown"), 1);

        // Remove another icon
        let out = run_cli(&["remove", "arrow-down", dir.to_str().unwrap()]);
        assert!(out.status.success());
        assert!(stdout_to_string(&out).contains("Removed"));

        let file_path = dir.join("arrows.ts");

        assert_eq!(file_contains(&file_path, "ArrowUp"), 0);
        assert_eq!(file_contains(&file_path, "ArrowDown"), 0);
    }

    #[test]
    fn test_add_to_file_with_duplicated_entries_deduplicates() {
        let temp_dir = create_temp_dir("test-icons-", PathBuf::from("tests"));
        let dir = temp_dir.path();

        std::fs::write(
            dir.join("arrows.ts"),
            "export {\n  ArrowUp,\n  ArrowUp,\n} from \"lucide-react\";\n",
        )
        .unwrap();

        let out = run_cli(&["add", "arrow-down", dir.to_str().unwrap()]);
        assert!(out.status.success());
        assert!(stdout_to_string(&out).contains("Added"));

        let file_path = dir.join("arrows.ts");
        assert_eq!(file_contains(&file_path, "ArrowUp"), 1);
        assert_eq!(file_contains(&file_path, "ArrowDown"), 1);
    }

    #[test]
    fn test_remove_from_file_with_duplicated_entries_deduplicates() {
        let temp_dir = create_temp_dir("test-icons-", PathBuf::from("tests"));
        let dir = temp_dir.path();

        std::fs::write(
            dir.join("arrows.ts"),
            "export {\n  ArrowUp,\n  ArrowUp,\n} from \"lucide-react\";\n",
        )
        .unwrap();

        let out = run_cli(&["remove", "arrow-up", dir.to_str().unwrap()]);
        assert!(out.status.success());
        assert!(stdout_to_string(&out).contains("Removed"));

        assert!(!dir.join("arrows.ts").exists());
    }

    #[test]
    fn test_remove_icon_twice() {
        let temp_dir = create_temp_dir("test-icons-", PathBuf::from("tests"));
        let dir = temp_dir.path();

        std::fs::write(
            dir.join("arrows.ts"),
            r#"export { ArrowUp } from './icons/ArrowUp';"#,
        )
        .unwrap();

        // Remove one icon
        let out = run_cli(&["remove", "arrow-up", dir.to_str().unwrap()]);
        assert!(out.status.success());
        assert!(stdout_to_string(&out).contains("Removed"));

        let file_path = dir.join("arrows.ts");

        assert_eq!(file_contains(&file_path, "ArrowUp"), 0);

        // Remove icon another time
        let out = run_cli(&["remove", "arrow-up", dir.to_str().unwrap()]);
        assert!(out.status.success());
        assert!(stdout_to_string(&out).contains("not found"));

        let file_path = dir.join("arrows.ts");

        assert_eq!(file_contains(&file_path, "export"), 0);
        assert_eq!(file_contains(&file_path, "lucide-react"), 0);
        assert_eq!(file_contains(&file_path, "ArrowUp"), 0);
    }
}

fn run_cli(args: &[&str]) -> std::process::Output {
    std::process::Command::new("cargo")
        .args(["run", "--"].iter().chain(args))
        .output()
        .unwrap()
}

fn stdout_to_string(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn file_contains(path: &Path, word: &str) -> usize {
    std::fs::read_to_string(path)
        .map(|content| content.matches(word).count())
        .unwrap_or(0)
}
