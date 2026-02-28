use std::io::{self, Write};

/// Prompts the user for input and returns the trimmed string.
pub fn prompt_user(msg: &str) -> io::Result<String> {
    print!("{}", msg);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("arrow-up"), "ArrowUp");
        assert_eq!(to_pascal_case("a-arrow-up"), "AArrowUp");
    }
}
