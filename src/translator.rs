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
    /// Initializes the translator.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let data = { hi: [ "hello", "hola", "hey" ] };
    /// let translator = new Translator(data, false);
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(dictionary: JsValue, auto_commit: bool) -> Result<Translator, String> {
        let dictionary: IndexMap<String, Vec<String>> = serde_wasm_bindgen::from_value(dictionary)
            .map_err(|err| format!("[translator] Invalid dictionary.\nCaused by:\n\t{err}."))?;

        Ok(Self {
            engine: NativeTranslator::new(dictionary, auto_commit),
        })
    }

    /// Register a translator from source code.
    ///
    /// Note that the translator is written using [The Rhai Script Language](https://rhai.rs).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let translator = new Translator({}, false);
    /// translator.register(
    ///     count_script,
    ///     "fn translate(input) {" +
    ///     "    return [input, "", input.len().to_string(), false];" +
    ///     "}"
    /// );
    ///
    /// translator.translate("hello") == [["hello", "", ["5"], false]];
    /// ```
    #[cfg(feature = "rhai")]
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

    /// Unregister a translator.
    #[cfg(feature = "rhai")]
    pub fn unregister(&mut self, name: &str) {
        self.engine.unregister(name);
    }

    /// Generate predicates based on the input.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let data = { hi: [ "hello" ] };
    /// let translator = new Translator(data, false);
    /// translator.translate("hi") == ["hi", "", ["hello"], true];
    /// ```
    pub fn translate(&self, input: &str) -> JsValue {
        serde_wasm_bindgen::to_value(&self.engine.translate(input)).unwrap()
    }
}
