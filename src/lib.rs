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
#[wasm_bindgen]
pub fn convert_toml_to_json(content: &str) -> Result<JsValue, String> {
    let data: toml::Value = toml::from_str(content)
        .map_err(|err| format!("Invalid toml data.\nCaused by:\n\t{err}."))?;
    let json = serde_json::to_string_pretty(&data)
        .map_err(|err| format!("Error while converting in json.\nCaused by:\n\t{err}."))?;

    Ok(JsValue::from_str(&json))
}
