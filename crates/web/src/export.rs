//! Getting the ascii back out of the page: clipboard and .txt download.

use js_sys::Array;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

/// Copies the ascii to the system clipboard.
///
/// # Errors
///
/// Returns a message suitable for display if the browser refuses the write,
/// which it does unless the page is on https or localhost.
pub async fn copy(art: &str) -> Result<(), String> {
    let clipboard = window().navigator().clipboard();

    JsFuture::from(clipboard.write_text(art))
        .await
        .map(|_| ())
        .map_err(|err| format!("could not copy to clipboard: {}", describe(&err)))
}

/// Offers the ascii to the user as a `.txt` download, matching what the cli
/// writes to disk.
///
/// # Errors
///
/// Returns a message suitable for display if the browser rejects the blob.
pub fn download(name: &str, art: &str) -> Result<(), String> {
    let parts = Array::of1(&JsValue::from_str(art));
    let options = BlobPropertyBag::new();
    options.set_type("text/plain;charset=utf-8");

    let blob = Blob::new_with_str_sequence_and_options(&parts, &options)
        .map_err(|err| format!("could not build the download: {}", describe(&err)))?;
    let url = Url::create_object_url_with_blob(&blob)
        .map_err(|err| format!("could not build the download: {}", describe(&err)))?;

    let anchor: HtmlAnchorElement = window()
        .document()
        .expect("document")
        .create_element("a")
        .map_err(|err| format!("could not build the download: {}", describe(&err)))?
        .unchecked_into();

    anchor.set_href(&url);
    anchor.set_download(&format!("{name}.txt"));
    anchor.click();

    // The blob stays alive until revoked, and the click has already read it.
    let _ = Url::revoke_object_url(&url);

    Ok(())
}

fn window() -> web_sys::Window {
    web_sys::window().expect("a browser window")
}

/// Best effort human readable form of a thrown `JsValue`.
fn describe(err: &JsValue) -> String {
    err.as_string()
        .or_else(|| {
            err.dyn_ref::<js_sys::Error>()
                .map(|err| String::from(err.message()))
        })
        .unwrap_or_else(|| "unknown error".to_owned())
}
