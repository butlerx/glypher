# textify

> Every project needs a logo, so why not an ascii one.

A Rust rewrite of [asciify](https://github.com/butlerx/asciify). Converts PNG and
JPEG images into ascii art, from the command line or in the browser.

```
$ textify --print octocat.png

           :,            ~*~:                                          :~==
           ~            ~MMMMN87=                                  ~IONMMMM?
           ~,           ZMMMMMMMMNZ=        ,,::::::::,,        ~7DMMMMMMMMN
           :,           NMMMMMMMMMMMDII$ZO8DDDNNNNNNNNNNDD8O$II8MMMMMMMMMMMM=
```

## Layout

| Crate                            | What it is                                            |
| -------------------------------- | ----------------------------------------------------- |
| [`asciify`](crates/asciify) | The conversion library. No io, no network, wasm ready. |
| [`cli`](crates/cli)         | The `textify` command line tool.                       |
| [`web`](crates/web)         | A wasm frontend built with Leptos and Trunk.           |

## Install

```sh
cargo install --path crates/cli
```

## Usage

```sh
textify --print octocat.png                 # write the art to stdout
textify octocat.png                         # save it as ./octocat.txt
textify --path docs octocat.png             # save it as docs/octocat.txt
textify --width 200 octocat.png             # wider output, more detail
textify --readme README.md octocat.png      # inject it into a readme
textify https://example.com/logo.png        # fetch a remote image
```

| Flag              | Default           | Description                                     |
| ----------------- | ----------------- | ----------------------------------------------- |
| `-p`, `--print`   | off               | Print to stdout instead of writing a file.       |
| `-r`, `--readme`  |                   | Replace `{{ . }}` in a markdown file with the art. |
| `--path`          | current directory | Directory to write the `.txt` file to.           |
| `-w`, `--width`   | `100`             | Width of the output in characters.               |

### Injecting into a readme

Put a `{{ . }}` placeholder where the art should go, then point `--readme` at
the file. The placeholder is replaced with a fenced code block, so running it
twice needs the placeholder put back first.

```md
# my project

{{ . }}
```

## Web frontend

```sh
rustup target add wasm32-unknown-unknown
cd crates/web
trunk serve            # http://localhost:8080
trunk build --release  # static files in crates/web/dist
```

Everything runs client side: the image never leaves the browser. Pasting a
remote url is subject to the host's CORS headers, so many image hosts will
refuse.

## Development

```sh
cargo test --workspace
cargo clippy --all-features --all-targets -- -D warnings
cargo build -p web --target wasm32-unknown-unknown
```

Git hooks are managed with [prek](https://prek.j178.dev): `prek install`.
Commit messages follow [conventional commits](https://www.conventionalcommits.org).

## Differences from asciify

The output is the same art, but a few rough edges got filed off:

- **Transparency composites onto white.** asciify premultiplied onto black, so
  a logo with a cut out background came out as a solid block of `M`.
- **Url detection checks the scheme.** asciify used `url.ParseRequestURI`, which
  called `/tmp/logo.png` a url and tried to fetch it over http.
- **`--readme` fails loudly** when there is no `{{ . }}` placeholder. asciify's
  `html/template` silently truncated the readme to nothing.
- **Output filenames strip url query strings**, so `logo.png?raw=1` saves as
  `logo.txt`.

Resampling uses the `image` crate's Lanczos3 rather than `nfnt/resize`. Around
4% of characters land one step off along the ramp; the art is otherwise
identical.

## License

MIT. See [LICENSE](LICENSE).
