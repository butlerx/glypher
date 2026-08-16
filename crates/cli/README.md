# cli

The `textify` command line tool. Wraps [`asciify`](../asciify) with file and url
loading, and the two output modes.

```sh
cargo install --path .
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

| Flag             | Default           | Description                                        |
| ---------------- | ----------------- | -------------------------------------------------- |
| `-p`, `--print`  | off               | Print to stdout instead of writing a file.          |
| `-r`, `--readme` |                   | Replace `{{ . }}` in a markdown file with the art.  |
| `--path`         | current directory | Directory to write the `.txt` file to.              |
| `-w`, `--width`  | `100`             | Width of the output in characters.                  |

`--print` wins over `--readme`, which wins over writing a `.txt` file.

## Modules

| File                             | What it does                                          |
| -------------------------------- | ----------------------------------------------------- |
| [`main.rs`](src/main.rs)     | Argument parsing and the run flow.                     |
| [`source.rs`](src/source.rs) | Opens a local path or fetches an `http(s)` url.        |
| [`output.rs`](src/output.rs) | Writes the `.txt` file, or injects into a readme.      |
| [`error.rs`](src/error.rs)   | One `thiserror` enum; every variant names what failed. |

Errors carry the path or url that caused them:

```
error: /nope/logo.png: No such file or directory (os error 2)
error: could not fetch https://example.invalid/logo.png: io: failed to lookup address information
error: no `{{ . }}` placeholder found in README.md
```
