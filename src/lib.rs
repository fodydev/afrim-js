#![deny(missing_docs)]
//! Binding of the afrim input method engine.
//!
mod preprocessor;
mod translator;

pub use preprocessor::Preprocessor;
pub use translator::Translator;

use toml::{self};
use wasm_bindgen::prelude::*;

/// Convert TOML to JSON.
///
/// # Example
///
/// ```ignore
/// let data = convertTomlToJson(
///     "[info]\n" +
///     "name = \"sample\"\n" +
///     "\n" +
///     "[data]\n" +
///     "hello = \"hi\"\n"
/// );
/// ```
#[wasm_bindgen(js_name = convertTomlToJson)]
pub fn convert_toml_to_json(content: &str) -> Result<JsValue, String> {
    let data: toml::Value = toml::from_str(content)
        .map_err(|err| format!("Invalid toml data.\nCaused by:\n\t{err}."))?;
    let json = serde_json::to_string(&data).unwrap();

    Ok(JsValue::from_str(&json))
}
