use std::path::{Path, PathBuf};

use tempfile::{Builder, TempDir};

/// Converts kebab-case to PascalCase (e.g., "arrow-up" -> "ArrowUp").
pub fn to_pascal_case(s: &str) -> String {
    s.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

pub fn _get_testing_directory(path: Vec<String>) -> String {
    let mut path_iter = path.iter();
    let Some(initial_path) = path_iter.next() else {
        return String::new();
    };

    let mut result = Path::new(initial_path).to_path_buf();

    while let Some(another_path) = path_iter.next() {
        result = result.join(format!("{another_path}"));
    }

    result.to_str().unwrap().to_owned()
}

pub fn create_temp_dir(prefix: &str, dir_in: PathBuf) -> TempDir {
    Builder::new().prefix(prefix).tempdir_in(dir_in).unwrap()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("arrow-up"), "ArrowUp");
        assert_eq!(to_pascal_case("a-arrow-up"), "AArrowUp");
    }

    #[test]
    fn test_get_testing_directory() {
        let result =
            _get_testing_directory(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        let expected = Path::new("a").join("b").join("c");
        assert_eq!(result, expected.to_str().unwrap());
    }

    #[test]
    fn test_get_empty_testing_directory() {
        let result = _get_testing_directory(vec![]);
        assert_eq!(result, String::from(""));
    }

    #[test]
    fn test_get_simple_testing_directory() {
        let result = _get_testing_directory(vec!["icons".to_string()]);
        let expected = Path::new("icons").to_str().unwrap();
        assert_eq!(result, expected);
    }
}
