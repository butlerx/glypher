//! The single page: drop an image, watch it turn into text.

use leptos::{ev, prelude::*};
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, HtmlInputElement};

use crate::state::{MAX_WIDTH, MIN_WIDTH, State};

#[component]
pub fn App() -> impl IntoView {
    let state = State::new();

    view! {
        <main>
            <header>
                <h1>"textify"</h1>
                <p>"Turn an image into ascii art. Runs entirely in your browser."</p>
            </header>

            <Dropzone state />
            <UrlBar state />
            <Controls state />

            {move || state.status.get().map(|message| view! { <p class="error">{message}</p> })}

            {move || match state.art() {
                Err(message) => view! { <p class="error">{message}</p> }.into_any(),
                Ok(art) if art.is_empty() => ().into_any(),
                Ok(art) => view! { <pre>{art}</pre> }.into_any(),
            }}
        </main>
    }
}

/// Drag and drop target, doubling as the file picker.
#[component]
fn Dropzone(state: State) -> impl IntoView {
    let dragging = RwSignal::new(false);

    let on_drop = move |ev: DragEvent| {
        ev.prevent_default();
        dragging.set(false);

        if let Some(file) = ev
            .data_transfer()
            .and_then(|data| data.files())
            .and_then(|files| files.get(0))
        {
            state.load_file(file);
        }
    };

    let on_pick = move |ev: ev::Event| {
        let input: HtmlInputElement = ev.target().expect("event target").unchecked_into();

        if let Some(file) = input.files().and_then(|files| files.get(0)) {
            state.load_file(file);
        }
    };

    view! {
        <section
            class="dropzone"
            class:dragging=move || dragging.get()
            on:dragover=move |ev: DragEvent| {
                ev.prevent_default();
                dragging.set(true);
            }
            on:dragleave=move |_| dragging.set(false)
            on:drop=on_drop
        >
            <p>"Drop an image here"</p>
            <label class="button">
                "Choose a file"
                <input type="file" accept="image/png,image/jpeg" on:change=on_pick />
            </label>
        </section>
    }
}

/// Fetches a remote image. Subject to the host's CORS headers.
#[component]
fn UrlBar(state: State) -> impl IntoView {
    let url = RwSignal::new(String::new());

    view! {
        <section class="url">
            <input
                type="url"
                placeholder="…or paste an image url"
                prop:value=move || url.get()
                on:input=move |ev| url.set(event_target_value(&ev))
                on:keydown=move |ev: ev::KeyboardEvent| {
                    if ev.key() == "Enter" {
                        url.with(|url| state.load_url(url));
                    }
                }
            />
            <button on:click=move |_| url.with(|url| state.load_url(url))>"Fetch"</button>
        </section>
    }
}

/// Width slider and the two export buttons, hidden until an image is loaded.
#[component]
fn Controls(state: State) -> impl IntoView {
    view! {
        <section class="controls" class:hidden=move || !state.has_image()>
            <label for="width">"Width " {move || state.width.get()}</label>
            <input
                id="width"
                type="range"
                min=MIN_WIDTH
                max=MAX_WIDTH
                prop:value=move || state.width.get()
                on:input=move |ev| {
                    if let Ok(width) = event_target_value(&ev).parse() {
                        state.width.set(width);
                    }
                }
            />
            <button on:click=move |_| state.copy()>"Copy"</button>
            <button on:click=move |_| state.download()>"Download .txt"</button>
        </section>
    }
}
