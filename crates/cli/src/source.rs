use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor, Read, Seek, SeekFrom},
    path::Path,
};

use crate::error::Error;

/// A seekable reader over a local or remote image.
pub enum Source {
    Local(BufReader<File>),
    Remote(Cursor<Vec<u8>>),
}

impl Read for Source {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::Local(reader) => reader.read(buf),
            Self::Remote(reader) => reader.read(buf),
        }
    }
}

impl BufRead for Source {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        match self {
            Self::Local(reader) => reader.fill_buf(),
            Self::Remote(reader) => reader.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            Self::Local(reader) => reader.consume(amt),
            Self::Remote(reader) => reader.consume(amt),
        }
    }
}

impl Seek for Source {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match self {
            Self::Local(reader) => reader.seek(pos),
            Self::Remote(reader) => reader.seek(pos),
        }
    }
}

/// Opens a reader for a remote or local file.
///
/// # Errors
///
/// Returns [`Error::Http`] if the request fails, or [`Error::Io`] if the local
/// file cannot be opened.
pub fn get_file(path: &str) -> Result<Source, Error> {
    if is_http_url(path) {
        let body = ureq::get(path)
            .call()
            .and_then(|mut res| res.body_mut().with_config().limit(u64::MAX).read_to_vec())
            .map_err(|err| Error::http(path, err))?;

        return Ok(Source::Remote(Cursor::new(body)));
    }

    let file = File::open(Path::new(path)).map_err(|err| Error::io(path, err))?;

    Ok(Source::Local(BufReader::new(file)))
}

/// Only http(s) urls are fetched; everything else is treated as a local path.
fn is_http_url(path: &str) -> bool {
    path.starts_with("http://") || path.starts_with("https://")
}

#[cfg(test)]
mod tests {
    use super::is_http_url;

    #[test]
    fn accepts_http_urls() {
        assert!(is_http_url("http://example.com/test?arg=1&arg=2"));
        assert!(is_http_url("https://example.com/logo.png"));
    }

    #[test]
    fn rejects_paths() {
        assert!(!is_http_url("Not a url"));
        assert!(!is_http_url("/tmp/logo.png"));
        assert!(!is_http_url("./logo.png"));
    }
}
