#![deny(missing_docs)]
/// Binding of the afrim preprocessor.
///
use afrim_preprocessor::Preprocessor as NativePreprocessor;
use indexmap::IndexMap;
use serde_wasm_bindgen::{self};
use wasm_bindgen::prelude::*;

/// Core structure of the preprocessor.
#[wasm_bindgen]
pub struct Preprocessor {
    engine: NativePreprocessor,
}

#[wasm_bindgen]
impl Preprocessor {
    /// Initializes the preprocessor.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let data = { a1: "à", ae: "æ" };
    /// let preprocessor = new Preprocessor(data, 64);
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(data: &JsValue, buffer_size: usize) -> Result<Preprocessor, String> {
        let data: IndexMap<String, String> = serde_wasm_bindgen::from_value(data.clone())
            .map_err(|err| format!("[preprocessor] Invalid data.\nCaused by:\n\t{err}."))?;
        let data = data
            .iter()
            .map(|(k, v)| vec![k.as_str(), v.as_str()])
            .collect();
        let map = utils::build_map(data);

        Ok(Self {
            engine: NativePreprocessor::new(map.into(), buffer_size),
        })
    }

    /// Process an keyboard event.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let data = { a1: "à", ae: "æ" };
    /// let preprocessor = new Preprocessor(data, 64);
    /// preprocessor.process("a", "keydown");
    /// preprocessor.process("1", "keydown");
    /// ```
    pub fn process(&mut self, key: &str, state: &str) -> Result<bool, String> {
        let key_event = utils::deserialize_event(key, state)?;
        let (changed, _) = self.engine.process(key_event);

        Ok(changed)
    }

    /// Commit a text.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let preprocessor = new Preprocessor(data, 64);
    /// preprocessor.commit("ŋna");
    /// ```
    pub fn commit(&mut self, text: String) {
        self.engine.commit(text);
    }

    /// Return the next command to be executed.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let data = { a1: "à", ae: "æ" };
    /// let preprocessor = new Preprocessor(data, 64);
    /// preprocessor.process("a", "keydown");
    /// preprocessor.process("1", "keydown");
    ///
    /// preprocessor.popQueue() == "Pause";
    /// preprocessor.popQueue() == { CommitText: "à" };
    /// preprocessor.popQueue() == "Resume";
    /// preprocessor.popQueue() == "NOP";
    /// ```
    #[wasm_bindgen(js_name = popQueue)]
    pub fn pop_queue(&mut self) -> String {
        self.engine
            .pop_queue()
            .as_ref()
            .map(utils::serialize_command)
            .unwrap_or("\"NOP\"".to_owned())
    }

    /// Return the input from the memory.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let preprocessor = new Preprocessor(data, 64);
    /// preprocessor.process("a", "keydown");
    /// preprocessor.process("1", "keydown");
    ///
    /// preprocessor.getInput() == "a1";
    /// ```
    #[wasm_bindgen(js_name = getInput)]
    pub fn get_input(&self) -> String {
        self.engine.get_input()
    }

    /// Clear the preprocessor commands from the queue.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let data = { a1: "à", ae: "æ" };
    /// let preprocessor = new Preprocessor(data, 64);
    /// preprocessor.process("a", "keydown");
    /// preprocessor.process("1", "keydown");
    ///
    /// preprocessor.clearQueue();
    /// preprocessor.popQueue() == "NOP";
    /// ```
    #[wasm_bindgen(js_name = clearQueue)]
    pub fn clear_queue(&mut self) {
        self.engine.clear_queue();
    }
}

pub mod utils {
    pub use afrim_preprocessor::utils::*;
    use afrim_preprocessor::{Command, Key, KeyState, KeyboardEvent};
    use std::str::FromStr;

    /// Convert an JsKeyboardEvent to KeyboardEvent.
    pub fn deserialize_event(key: &str, state: &str) -> Result<KeyboardEvent, String> {
        let event = KeyboardEvent {
            key: Key::from_str(key)
                .map_err(|err| format!("[preprocessor] Unrecognized key.\nCaused by:\n\t{err}."))?,
            state: match state {
                "keydown" => KeyState::Down,
                "keyup" => KeyState::Up,
                _ => return Err(format!("[preprocessor] Unrecognized state `{state}`.")),
            },
            ..Default::default()
        };

        Ok(event)
    }

    /// Convert a preprocessor command to speudo code.
    pub fn serialize_command(command: &Command) -> String {
        serde_json::to_string(command).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::preprocessor::utils;

    #[test]
    fn test_deserialize_event() {
        use afrim_preprocessor::{Key, KeyState, KeyboardEvent};

        assert_eq!(
            utils::deserialize_event("Alt", "keydown").unwrap(),
            KeyboardEvent {
                key: Key::Alt,
                state: KeyState::Down,
                ..Default::default()
            }
        );
        assert_eq!(
            utils::deserialize_event(" ", "keyup").unwrap(),
            KeyboardEvent {
                key: Key::Character(" ".to_owned()),
                state: KeyState::Up,
                ..Default::default()
            }
        );
        assert!(utils::deserialize_event("key", "keyup").is_err());
        assert!(utils::deserialize_event("a", "up").is_err());
    }

    #[test]
    fn test_serialize_command() {
        use afrim_preprocessor::Command;

        assert_eq!(
            utils::serialize_command(&Command::CommitText("text".to_string())),
            "{\"CommitText\":\"text\"}".to_string()
        );
        assert_eq!(
            utils::serialize_command(&Command::Pause),
            "\"Pause\"".to_string()
        );
        assert_eq!(
            utils::serialize_command(&Command::Resume),
            "\"Resume\"".to_string()
        );
        assert_eq!(
            utils::serialize_command(&Command::Delete),
            "\"Delete\"".to_string()
        );
    }
}
