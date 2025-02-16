use std::collections::HashMap;
use std::str::FromStr;
use device_query::{mouse_state, CallbackGuard, Keycode};
use serde::{Deserialize, Serialize};
use tauri::ipc::Channel;

#[derive(Clone, PartialEq, Hash, Eq)]
pub enum KeyType {
    Keyboard(Keycode),
    Mouse(mouse_state::MouseButton),
}

pub struct Shortcut {
    inner: HashMap<KeyType, bool>,
    pub channel: Channel<ShortcutEvent>,
}

impl Shortcut {
    pub fn new(keys: Vec<KeyType>, channel: Channel<ShortcutEvent>) -> Self {
        Self {
            inner: keys.into_iter().map(|x| (x, false)).collect(),
            channel,
        }
    }

    /// Returns true if Shortcut activated now
    pub fn down(&mut self, key: &KeyType) -> bool {
        if self.inner.contains_key(key) && !self.inner[key] {
            self.inner.insert(key.clone(), true);
            self.inner.iter().all(|(_, v)| *v)
        } else {
            false
        }
    }

    /// Returns true if Shortcut deactivated now
    pub fn up(&mut self, key: &KeyType) -> bool {
        if self.inner.contains_key(key) && self.inner[key] {
            let all_pressed = self.inner.iter().all(|(_, v)| *v);
            self.inner.insert(key.clone(), false);
            all_pressed
        } else {
            false
        }
    }
}

impl From<&str> for KeyType {
    fn from(value: &str) -> Self {
        if value.starts_with("Mouse") {
            let n: usize = value["Mouse".len()..].trim().parse().unwrap();
            Self::Mouse(n)
        } else {
            Self::Keyboard(Keycode::from_str(value).unwrap())
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ShortcutEvent {
    pub name: String,
    pub state: ShortcutState,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum ShortcutState {
    Pressed,
    Released,
}