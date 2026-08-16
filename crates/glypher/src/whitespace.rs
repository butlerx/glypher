/// Trims trailing spaces from every line, then drops leading and trailing
/// blank lines.
#[must_use]
pub fn trim_whitespace(txt: &str) -> String {
    let trimmed = txt
        .lines()
        .map(|line| line.trim_end_matches(' '))
        .collect::<Vec<_>>();

    match (
        trimmed.iter().position(|line| !line.is_empty()),
        trimmed.iter().rposition(|line| !line.is_empty()),
    ) {
        (Some(start), Some(end)) => trimmed[start..=end].join("\n"),
        _ => trimmed.join("\n"),
    }
}
