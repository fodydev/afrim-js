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
    /// Initiate the preprocessor.
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

    /// Process the keyboard event.
    pub fn process(&mut self, key: String, state: String) -> Result<bool, String> {
        let key_event = utils::parse_event(key, state)?;
        let (changed, _) = self.engine.process(key_event);

        Ok(changed)
    }

    /// Commit the text.
    pub fn commit(&mut self, text: String) {
        self.engine.commit(text);
    }

    /// Return the next command to be executed.
    pub fn pop_queue(&mut self) -> String {
        self.engine
            .pop_queue()
            .map(utils::parse_command)
            .unwrap_or(".".to_owned())
    }

    /// Return the input from the memory.
    pub fn get_input(&self) -> String {
        self.engine.get_input()
    }

    /// Clear the preprocessor commands from the queue.
    pub fn clear_queue(&mut self) {
        self.engine.clear_queue();
    }
}

pub mod utils {
    pub use afrim_preprocessor::utils::*;
    use afrim_preprocessor::{Command, Key, KeyState, KeyboardEvent};
    use std::str::FromStr;

    /// Convert an JsKeyboardEvent to KeyboardEvent.
    pub fn parse_event(key: String, state: String) -> Result<KeyboardEvent, String> {
        let event = KeyboardEvent {
            key: Key::from_str(&key)
                .map_err(|err| format!("[preprocessor] Unrecognized key.\nCaused by:\n\t{err}."))?,
            state: match state.as_str() {
                "keydown" => KeyState::Down,
                "keyup" => KeyState::Up,
                _ => Default::default(),
            },
            ..Default::default()
        };

        Ok(event)
    }

    /// Convert a preprocessor command to speudo code.
    pub fn parse_command(command: Command) -> String {
        serde_json::to_string(&command).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::preprocessor::utils;

    #[test]
    fn test_from_event() {
        use afrim_preprocessor::{Key, KeyState, KeyboardEvent};

        assert_eq!(
            utils::parse_event("Alt".to_owned(), "keydown".to_owned()).unwrap(),
            KeyboardEvent {
                key: Key::Alt,
                state: KeyState::Down,
                ..Default::default()
            }
        );
        assert_eq!(
            utils::parse_event(" ".to_owned(), "keyup".to_owned()).unwrap(),
            KeyboardEvent {
                key: Key::Character(" ".to_owned()),
                state: KeyState::Up,
                ..Default::default()
            }
        );
        assert!(utils::parse_event("key".to_string(), "up".to_string()).is_err());
    }

    #[test]
    fn test_parse_command() {
        use afrim_preprocessor::Command;

        assert_eq!(
            utils::parse_command(Command::CommitText("text".to_string())),
            "{\"CommitText\":\"text\"}".to_string()
        );
        assert_eq!(
            utils::parse_command(Command::Pause),
            "\"Pause\"".to_string()
        );
        assert_eq!(
            utils::parse_command(Command::Resume),
            "\"Resume\"".to_string()
        );
        assert_eq!(
            utils::parse_command(Command::Delete),
            "\"Delete\"".to_string()
        );
    }
}
