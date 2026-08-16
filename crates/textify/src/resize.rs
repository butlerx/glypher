use image::{DynamicImage, GenericImageView, imageops::FilterType};

/// Terminal cells are taller than they are wide, so squash the height to keep
/// the aspect ratio looking right once rendered as text.
const CELL_WIDTH: u64 = 10;
const CELL_HEIGHT: u64 = 16;

/// Resizes the image to the final width while maintaining aspect ratio.
#[must_use]
pub fn resize(img: &DynamicImage, width: u32) -> DynamicImage {
    let (w, h) = img.dimensions();
    if w == 0 || h == 0 {
        return img.clone();
    }

    let height = (u64::from(h) * u64::from(width) * CELL_WIDTH) / (u64::from(w) * CELL_HEIGHT);
    let height = u32::try_from(height).unwrap_or(u32::MAX).max(1);

    img.resize_exact(width.max(1), height, FilterType::Lanczos3)
}
