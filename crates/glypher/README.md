# glypher

The conversion library behind [glypher](../../README.md). Takes an image, gives
back ascii art. No io, no network, no threads, so it compiles to
`wasm32-unknown-unknown` unchanged.

## Usage

```rust
use std::{fs::File, io::BufReader};

let image = BufReader::new(File::open("octocat.png")?);
let art = glypher::generate(image, 100)?;

println!("{art}");
```

`generate` takes any `BufRead + Seek`, so a `Cursor<Vec<u8>>` works just as well
as a file — that is how the wasm frontend feeds it a dropped browser `File`.

## How it works

1. **[`resize`]** — scales the image to the requested width, squashing the
   height by `10/16` so the art keeps its aspect ratio once rendered in
   character cells that are taller than they are wide.
2. **[`pixel_to_char`]** — converts each pixel to Rec. 601 luma and picks a
   character of matching intensity from the 17 step ramp `MND8OZ$7I?*=~:,  `.
   Transparent pixels composite onto white, so a cut out background renders as
   blank space.
3. **[`trim_whitespace`]** — strips trailing spaces from every line, then drops
   leading and trailing blank lines.

Each step is public, so you can swap in your own pipeline.

## Errors

`generate` returns [`Error`], which is either an io failure reading the source
or a decode failure from the `image` crate.

[`resize`]: src/resize.rs
[`pixel_to_char`]: src/pixel2char.rs
[`trim_whitespace`]: src/whitespace.rs
[`Error`]: src/error.rs
