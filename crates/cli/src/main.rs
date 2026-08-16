#![warn(clippy::pedantic)]

use crate::error::Error;
use clap::Parser;
use std::{
    env,
    path::{Path, PathBuf},
    process::ExitCode,
};
use glypher::generate;

mod error;
mod output;
mod source;

const DEFAULT_WIDTH: u32 = 100;

/// Every project needs a logo, so why not an ascii one.
#[derive(Parser)]
#[command(
    name = "glypher",
    about = "glypher all the things",
    long_about = "Every project needs a logo, so why not an ascii one.\n\
                  Convert an image to ascii.\n\
                  Supports injecting the image into a readme.",
    version
)]
struct Cli {
    /// Path or url of the image to convert
    image: String,

    /// Path to readme.md to inject the ascii into
    #[arg(short, long, value_name = "FILE")]
    readme: Option<PathBuf>,

    /// Directory to write the txt file to
    #[arg(long, value_name = "DIR")]
    path: Option<PathBuf>,

    /// Print the image to the terminal
    #[arg(short, long)]
    print: bool,

    /// Width of the output in characters
    #[arg(short, long, default_value_t = DEFAULT_WIDTH)]
    width: u32,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match run(&cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: &Cli) -> Result<(), Error> {
    let img = generate(source::get_file(&cli.image)?, cli.width)?;

    // Handle print first (early return path)
    if cli.print {
        println!("{img}");
        return Ok(());
    }

    let action = OutputAction::from_cli(cli);
    action.execute(&cli.image, &img)
}

enum OutputAction {
    InjectReadme(PathBuf),
    SaveToFile(PathBuf),
}

impl OutputAction {
    fn from_cli(cli: &Cli) -> Self {
        if let Some(readme) = &cli.readme { Self::InjectReadme(readme.clone()) } else {
            let dir = env::current_dir()
                .map_err(|src| Error::Cwd { source: src })
                .unwrap_or_else(|_| env::current_dir().expect("cannot recover cwd"));
            let txt = Path::new(dir.as_path()).join(format!("{}.txt", stem(&cli.image)));
            Self::SaveToFile(txt)
        }
    }

    fn execute(self, image_name: &str, img: &str) -> Result<(), Error> {
        match self {
            Self::InjectReadme(path) => output::inject_readme(&path, img),
            Self::SaveToFile(path) => {
                output::save(&path, img)?;
                println!("{image_name} saved to {}", path.display());
                Ok(())
            }
        }
    }
}

/// File name of the image without its extension, url query strings included.
fn stem(image: &str) -> String {
    let name = image.split(['?', '#']).next().unwrap_or(image);

    Path::new(name)
        .file_stem()
        .map_or_else(|| "image".to_string(), |s| s.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::stem;

    #[test]
    fn strips_directories_and_extensions() {
        assert_eq!(stem("./assets/octocat.png"), "octocat");
        assert_eq!(stem("octocat.png"), "octocat");
    }

    #[test]
    fn strips_url_query_strings() {
        assert_eq!(stem("https://example.com/logo.png?raw=1"), "logo");
    }
}
