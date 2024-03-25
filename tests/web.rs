//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use serde_wasm_bindgen::{self};
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_process() {
    use afrim_js::Preprocessor;

    let mut data = HashMap::new();
    data.insert("a1".to_owned(), "à".to_owned());
    data.insert("e2".to_owned(), "é".to_owned());
    let map = serde_wasm_bindgen::to_value(&data).unwrap();

    // Process
    let mut preprocessor = Preprocessor::new(map, 32).unwrap();
    preprocessor.process("a", "keydown").unwrap();
    preprocessor.process("Backspace", "keydown").unwrap();
    assert_eq!(preprocessor.get_input(), "".to_owned());

    preprocessor.process("a", "keydown").unwrap();
    preprocessor.process("1", "keydown").unwrap();
    assert_eq!(preprocessor.get_input(), "a1".to_owned());

    // Get commands
    assert_eq!(preprocessor.pop_queue(), JsValue::from_str("Pause"));
    assert_eq!(preprocessor.pop_queue(), JsValue::from_str("CleanDelete"));
    assert_eq!(preprocessor.pop_queue(), JsValue::from_str("Resume"));
    assert_eq!(preprocessor.pop_queue(), JsValue::from_str("Pause"));
    assert_eq!(preprocessor.pop_queue(), JsValue::from_str("Delete"));
    assert_eq!(preprocessor.pop_queue(), JsValue::from_str("Delete"));
    assert!(preprocessor.pop_queue().is_object());
    assert_eq!(preprocessor.pop_queue(), JsValue::from_str("Resume"));
    assert_eq!(preprocessor.pop_queue(), JsValue::from_str("NOP"));
}

#[wasm_bindgen_test]
fn test_translate() {
    use afrim_js::Translator;

    let mut dictionary = HashMap::new();
    dictionary.insert("hello".to_owned(), vec!["hi".to_owned()]);
    dictionary.insert("hallo".to_owned(), vec!["hola".to_owned()]);
    let dictionary = serde_wasm_bindgen::to_value(&dictionary).unwrap();

    // Translate
    let translator = Translator::new(dictionary, false).unwrap();
    let translations: Vec<(String, String, Vec<String>, bool)> =
        serde_wasm_bindgen::from_value(translator.translate("hello")).unwrap();

    #[cfg(not(feature = "strsim"))]
    assert_eq!(
        translations,
        vec![(
            "hello".to_owned(),
            "".to_owned(),
            vec!["hi".to_owned()],
            false
        )]
    );

    #[cfg(feature = "strsim")]
    assert_eq!(
        translations,
        vec![
            (
                "hello".to_owned(),
                "".to_owned(),
                vec!["hi".to_owned()],
                false
            ),
            (
                "hallo".to_owned(),
                "".to_owned(),
                vec!["hola".to_owned()],
                false
            )
        ]
    );
}

#[cfg(feature = "rhai")]
#[wasm_bindgen_test]
fn test_transaltor() {
    use afrim_js::Translator;

    // Script
    let count_script = r#"fn translate(input) { [input, "", input.len().to_string(), false] }"#;

    // Translate
    let dictionary = serde_wasm_bindgen::to_value(&HashMap::<String, String>::new()).unwrap();
    let mut translator = Translator::new(dictionary, false).unwrap();
    translator
        .register("count".to_owned(), count_script.to_owned())
        .unwrap();
    let translations: Vec<(String, String, Vec<String>, bool)> =
        serde_wasm_bindgen::from_value(translator.translate("hello")).unwrap();

    assert_eq!(
        translations,
        vec![(
            "hello".to_owned(),
            "".to_owned(),
            vec!["5".to_owned()],
            false
        ),]
    );

    translator.unregister("count");
}

#[wasm_bindgen_test]
fn test_toml() {
    use afrim_js::convert_toml_to_json;

    let data = convert_toml_to_json("[data]\nhi = \"hello\"");
    assert!(data.unwrap().is_object());
}
