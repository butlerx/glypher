//! Shared reactive state. Every field is a signal, so `State` is `Copy` and
//! can be handed to each component by value.

use std::io::Cursor;

use leptos::{prelude::*, task::spawn_local};
use web_sys::File;

use crate::{export, source};

pub const DEFAULT_WIDTH: u32 = 100;
pub const MIN_WIDTH: u32 = 20;
pub const MAX_WIDTH: u32 = 300;

#[derive(Clone, Copy)]
pub struct State {
    image: RwSignal<Option<source::Image>>,
    /// Ok is the ascii art, Err is a decode failure to show the user.
    art: Memo<Result<String, String>>,
    pub width: RwSignal<u32>,
    pub status: RwSignal<Option<String>>,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    #[must_use]
    pub fn new() -> Self {
        let image = RwSignal::new(None::<source::Image>);
        let width = RwSignal::new(DEFAULT_WIDTH);

        // Re-runs whenever the image or the width changes. Conversion is fast
        // enough to keep up with the slider.
        let art = Memo::new(move |_| {
            image.with(|image| match image {
                None => Ok(String::new()),
                Some(image) => glypher::generate(Cursor::new(&image.bytes), width.get())
                    .map_err(|err| err.to_string()),
            })
        });

        Self {
            image,
            art,
            width,
            status: RwSignal::new(None),
        }
    }

    pub fn art(self) -> Result<String, String> {
        self.art.get()
    }

    #[must_use]
    pub fn has_image(self) -> bool {
        self.image.with(Option::is_some)
    }

    /// Reads a picked or dropped file into the page.
    pub fn load_file(self, file: File) {
        spawn_local(async move { self.accept(source::from_file(file).await) });
    }

    /// Fetches a remote image. Does nothing for a blank url.
    pub fn load_url(self, url: &str) {
        let url = url.trim().to_owned();
        if url.is_empty() {
            return;
        }

        spawn_local(async move { self.accept(source::from_url(&url).await) });
    }

    /// Copies the current art to the clipboard.
    pub fn copy(self) {
        let Ok(art) = self.art.get() else { return };

        spawn_local(async move {
            if let Err(err) = export::copy(&art).await {
                self.status.set(Some(err));
            }
        });
    }

    /// Downloads the current art as `<name>.txt`.
    pub fn download(self) {
        let Ok(art) = self.art.get() else { return };
        let name = self.image.with(|image| {
            image
                .as_ref()
                .map_or_else(|| "image".to_owned(), |image| image.name.clone())
        });

        if let Err(err) = export::download(&name, &art) {
            self.status.set(Some(err));
        }
    }

    fn accept(self, loaded: Result<source::Image, String>) {
        match loaded {
            Ok(image) => {
                self.status.set(None);
                self.image.set(Some(image));
            }
            Err(err) => self.status.set(Some(err)),
        }
    }
}
