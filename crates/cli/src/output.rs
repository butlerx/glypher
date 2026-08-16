use std::{fs, path::Path};

use crate::error::Error;

/// The token replaced by the ascii art, kept for compatibility with asciify's
/// go template placeholder.
const PLACEHOLDER: &str = "{{ . }}";
const PLACEHOLDER_COMPACT: &str = "{{.}}";

/// Writes the ascii art to a file, creating it if it doesn't exist.
///
/// # Errors
///
/// Returns [`Error::Io`] if the file cannot be written.
pub fn save(path: &Path, img: &str) -> Result<(), Error> {
    fs::write(path, img).map_err(|err| Error::io(path, err))
}

/// Injects the ascii art into a readme, replacing the `{{ . }}` placeholder
/// with a fenced code block.
///
/// # Errors
///
/// Returns [`Error::Io`] if the readme cannot be read or written, or
/// [`Error::NoPlaceholder`] if it contains no placeholder.
pub fn inject_readme(path: &Path, img: &str) -> Result<(), Error> {
    let readme = fs::read_to_string(path).map_err(|err| Error::io(path, err))?;
    let block = wrap_code(img);

    let injected = if readme.contains(PLACEHOLDER) {
        readme.replace(PLACEHOLDER, &block)
    } else if readme.contains(PLACEHOLDER_COMPACT) {
        readme.replace(PLACEHOLDER_COMPACT, &block)
    } else {
        return Err(Error::NoPlaceholder(path.to_path_buf()));
    };

    fs::write(path, injected).map_err(|err| Error::io(path, err))
}

fn wrap_code(code: &str) -> String {
    format!("\n```\n{code}\n```\n")
}

#[cfg(test)]
mod tests {
    use super::wrap_code;

    #[test]
    fn wraps_in_a_fence() {
        assert_eq!(wrap_code("art"), "\n```\nart\n```\n");
    }
}
