//! Loading image bytes from a local file or a remote url.

use gloo_file::{Blob, futures::read_as_bytes};
use gloo_net::http::Request;
use web_sys::File;

/// An image loaded into memory, along with the name to save it under.
pub struct Image {
    pub name: String,
    pub bytes: Vec<u8>,
}

/// Reads a file the user picked or dropped.
///
/// # Errors
///
/// Returns a message suitable for display if the file cannot be read.
pub async fn from_file(file: File) -> Result<Image, String> {
    let name = file.name();
    let bytes = read_as_bytes(&Blob::from(file))
        .await
        .map_err(|err| format!("could not read {name}: {err}"))?;

    Ok(Image {
        name: stem(&name),
        bytes,
    })
}

/// Fetches a remote image.
///
/// # Errors
///
/// Returns a message suitable for display if the request fails. Cross origin
/// hosts that do not send `Access-Control-Allow-Origin` will land here.
pub async fn from_url(url: &str) -> Result<Image, String> {
    let response = Request::get(url)
        .send()
        .await
        .map_err(|err| format!("could not fetch {url}: {err}"))?;

    if !response.ok() {
        return Err(format!("could not fetch {url}: HTTP {}", response.status()));
    }

    let bytes = response
        .binary()
        .await
        .map_err(|err| format!("could not read {url}: {err}"))?;

    Ok(Image {
        name: stem(url),
        bytes,
    })
}

/// File name without its directory, query string or extension.
fn stem(name: &str) -> String {
    let name = name.split(['?', '#']).next().unwrap_or(name);
    let name = name.rsplit('/').next().unwrap_or(name);

    name.rsplit_once('.')
        .map_or(name, |(stem, _)| stem)
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::stem;

    #[test]
    fn strips_directories_extensions_and_queries() {
        assert_eq!(stem("octocat.png"), "octocat");
        assert_eq!(stem("https://example.com/a/logo.png?raw=1"), "logo");
        assert_eq!(stem("noext"), "noext");
    }
}
