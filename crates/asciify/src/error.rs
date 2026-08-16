use thiserror::Error;

/// Everything that can go wrong turning an image into text.
#[derive(Debug, Error)]
pub enum Error {
    #[error("could not read image: {0}")]
    Io(#[from] std::io::Error),

    #[error("could not decode image: {0}")]
    Decode(#[from] image::ImageError),
}
