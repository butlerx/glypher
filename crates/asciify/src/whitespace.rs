/// Trims trailing spaces from every line, then drops leading and trailing
/// blank lines.
#[must_use]
pub fn trim_whitespace(txt: &str) -> String {
    let lines: Vec<&str> = txt.lines().map(|line| line.trim_end_matches(' ')).collect();

    let start = lines.iter().position(|line| !line.is_empty());
    let end = lines.iter().rposition(|line| !line.is_empty());

    match (start, end) {
        (Some(start), Some(end)) => lines[start..=end].join("\n"),
        _ => lines.join("\n"),
    }
}

#[cfg(test)]
mod tests {
    use super::trim_whitespace;

    #[test]
    fn trims_trailing_spaces() {
        assert_eq!(trim_whitespace("ab   \ncd  \n"), "ab\ncd");
    }

    #[test]
    fn drops_surrounding_blank_lines() {
        assert_eq!(trim_whitespace("  \n\nab\n\n   \n"), "ab");
    }

    #[test]
    fn keeps_interior_blank_lines() {
        assert_eq!(trim_whitespace("ab\n\ncd"), "ab\n\ncd");
    }

    #[test]
    fn all_blank_is_empty() {
        assert_eq!(trim_whitespace("   \n  \n"), "\n");
    }
}
