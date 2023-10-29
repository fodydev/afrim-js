#![deny(missing_docs)]
/// Binding of the afrim translator for wasm.
///
use afrim_translator::Translator as NativeTranslator;
use indexmap::IndexMap;
use serde_wasm_bindgen::{self};
use wasm_bindgen::prelude::*;

/// Core structure of the translator.
#[wasm_bindgen]
pub struct Translator {
    engine: NativeTranslator,
}

#[wasm_bindgen]
impl Translator {
    /// Initiate the translator.
    pub fn new(dictionary: &JsValue, auto_commit: bool) -> Self {
        let dictionary: IndexMap<String, Vec<String>> =
            serde_wasm_bindgen::from_value(dictionary.clone()).unwrap();

        Self {
            engine: NativeTranslator::new(dictionary, auto_commit),
        }
    }

    /// Generate predicates based on the input.
    pub fn translate(&self, input: &str) -> JsValue {
        serde_wasm_bindgen::to_value(&self.engine.translate(input)).unwrap()
    }
}
