# web

A wasm frontend for [`asciify`](../asciify), built with
[Leptos](https://leptos.dev) and [Trunk](https://trunkrs.dev). Drop an image in,
get ascii art out. The image never leaves the browser — there is no server.

## Running it

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk

trunk serve            # http://localhost:8080, rebuilds on save
trunk build --release  # static files in ./dist, ~585 KB of wasm
```

`dist/` is a plain static bundle: drop it on any host.

## What it does

- **Drag and drop, or a file picker.** PNG and JPEG.
- **Live width slider**, 20 to 300 characters. Conversion is fast enough to
  re-render on every drag.
- **Copy** to the clipboard, or **download** the art as `<name>.txt`, matching
  what the cli writes.
- **Paste an image url.** Subject to the host's CORS headers, so plenty of image
  hosts will refuse; the failure shows up as an error under the controls.

## Modules

| File                           | What it does                                            |
| ------------------------------ | -------------------------------------------------------- |
| [`main.rs`](src/main.rs)   | Mounts the app.                                           |
| [`app.rs`](src/app.rs)     | The page, split into `Dropzone`, `UrlBar` and `Controls`. |
| [`state.rs`](src/state.rs) | Shared signals plus the load, copy and download actions.  |
| [`source.rs`](src/source.rs) | Reads a browser `File`, or fetches a url.               |
| [`export.rs`](src/export.rs) | Clipboard writes and blob downloads.                    |

`State` is all signals, so it is `Copy` and gets passed to each component by
value rather than through context.

## Styling

[`style.css`](style.css) is hand written, no framework, and follows the
viewer's light or dark preference. Trunk picks it up via the `data-trunk` link
in [`index.html`](index.html).
