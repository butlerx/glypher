//! Convert images to ascii art.
#![warn(clippy::pedantic)]

mod error;
mod pixel2char;
mod resize;
mod whitespace;

use image::ImageReader;
use std::io::{BufRead, Seek};

pub use error::Error;
pub use pixel2char::pixel_to_char;
pub use resize::resize;
pub use whitespace::trim_whitespace;

/// Converts any image into an ascii image.
///
/// # Errors
///
/// Returns [`Error::Io`] if the reader fails, or [`Error::Decode`] if it does
/// not contain a decodable image.
pub fn generate<R: BufRead + Seek>(reader: R, width: u32) -> Result<String, Error> {
    let img = ImageReader::new(reader).with_guessed_format()?.decode()?;

    Ok(trim_whitespace(&pixel_to_char(&resize(&img, width))))
}
