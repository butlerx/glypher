use std::path::PathBuf;
use thiserror::Error;

/// Everything that can go wrong running the cli.
#[derive(Debug, Error)]
pub enum Error {
    #[error("{path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    // Deliberately `#[source]`, not `#[from]`: a blanket `From<io::Error>`
    // would relabel every io failure as a cwd lookup.
    #[error("could not determine the current directory: {source}")]
    Cwd {
        #[source]
        source: std::io::Error,
    },

    #[error(transparent)]
    Ascii(#[from] glypher::Error),

    #[error("could not fetch {url}: {source}")]
    Http {
        url: String,
        #[source]
        source: Box<ureq::Error>,
    },

    #[error("no `{{{{ . }}}}` placeholder found in {0}")]
    NoPlaceholder(PathBuf),
}

impl Error {
    /// Tags an io error with the path that caused it.
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }

    /// Tags a request error with the url that caused it.
    pub fn http(url: impl Into<String>, source: ureq::Error) -> Self {
        Self::Http {
            url: url.into(),
            source: Box::new(source),
        }
    }
}
