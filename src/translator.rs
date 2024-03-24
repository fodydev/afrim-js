#![deny(missing_docs)]
/// Binding of the afrim translator.
///
#[cfg(feature = "rhai")]
use afrim_translator::Engine;
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
    #[wasm_bindgen(constructor)]
    pub fn new(dictionary: &JsValue, auto_commit: bool) -> Result<Translator, String> {
        let dictionary: IndexMap<String, Vec<String>> =
            serde_wasm_bindgen::from_value(dictionary.clone())
                .map_err(|err| format!("[translator] Invalid dictionary.\nCaused by:\n\t{err}."))?;

        Ok(Self {
            engine: NativeTranslator::new(dictionary, auto_commit),
        })
    }

    #[cfg(feature = "rhai")]
    /// Register a translator from source code.
    pub fn register(&mut self, name: String, source: String) -> Result<(), String> {
        let engine = Engine::new_raw();
        let ast = engine.compile(source).map_err(|err| {
            format!(
                "[translator] Failed to register the translator `{name}`.\nCaused by:\n\t{err}."
            )
        })?;

        self.engine.register(name, ast);

        Ok(())
    }

    #[cfg(feature = "rhai")]
    /// Unregister a translator
    pub fn unregister(&mut self, name: &str) {
        self.engine.unregister(name);
    }

    /// Generate predicates based on the input.
    pub fn translate(&self, input: &str) -> JsValue {
        serde_wasm_bindgen::to_value(&self.engine.translate(input)).unwrap()
    }
}
