use image::{DynamicImage, GenericImageView};

/// Ramp from darkest to lightest. 17 entries so an 8 bit grey value scaled by
/// `16 / 255` indexes it directly.
const CHARS: &[u8; 17] = b"MND8OZ$7I?*=~:,  ";
const COLOUR_LEN: u32 = 255;

/// Converts each pixel to grey scale and replaces it with a character of
/// similar intensity.
#[must_use]
pub fn pixel_to_char(img: &DynamicImage) -> String {
    let (width, height) = img.dimensions();
    let mut buf = String::with_capacity(((width + 1) * height) as usize);

    for y in 0..height {
        for x in 0..width {
            let idx = u32::from(grey(img.get_pixel(x, y).0)) * 16 / COLOUR_LEN;
            buf.push(CHARS[idx as usize] as char);
        }

        buf.push('\n');
    }

    buf
}

/// Rec. 601 luma, using the same integer weights as Go's `color.GrayModel`.
///
/// Transparent pixels composite onto white, so a logo with a cut out
/// background renders as blank space rather than a solid block of `M`.
fn grey([r, g, b, a]: [u8; 4]) -> u8 {
    let a = u32::from(a);
    let over_white = |c: u8| (u32::from(c) * a + 255 * (255 - a)) / 255;
    let (r, g, b) = (over_white(r), over_white(g), over_white(b));

    let luma = (19595 * r + 38470 * g + 7471 * b + (1 << 15)) >> 16;

    u8::try_from(luma).unwrap_or(u8::MAX)
}

#[cfg(test)]
mod tests {
    use super::{grey, pixel_to_char};
    use image::{DynamicImage, Rgba, RgbaImage};

    #[test]
    fn transparent_image_renders_blank() {
        let img = DynamicImage::ImageRgba8(RgbaImage::from_pixel(3, 2, Rgba([0, 0, 0, 0])));

        assert_eq!(pixel_to_char(&img), "   \n   \n");
    }

    #[test]
    fn opaque_black_renders_the_darkest_char() {
        let img = DynamicImage::ImageRgba8(RgbaImage::from_pixel(2, 1, Rgba([0, 0, 0, 255])));

        assert_eq!(pixel_to_char(&img), "MM\n");
    }

    #[test]
    fn black_and_white() {
        assert_eq!(grey([0, 0, 0, 255]), 0);
        assert_eq!(grey([255, 255, 255, 255]), 255);
    }

    #[test]
    fn transparent_composites_onto_white() {
        assert_eq!(grey([0, 0, 0, 0]), 255);
        assert_eq!(grey([255, 255, 255, 0]), 255);
    }

    #[test]
    fn half_transparent_black_is_mid_grey() {
        let mid = grey([0, 0, 0, 128]);
        assert!((125..=130).contains(&mid), "got {mid}");
    }

    #[test]
    fn weights_green_heaviest() {
        assert!(grey([0, 255, 0, 255]) > grey([255, 0, 0, 255]));
        assert!(grey([255, 0, 0, 255]) > grey([0, 0, 255, 255]));
    }
}
