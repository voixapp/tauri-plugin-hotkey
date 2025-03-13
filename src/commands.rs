use crate::models::*;
use crate::State;
use tauri::{
    ipc::Channel,
    State as TauriState,
};

#[tauri::command]
pub fn register(
    state: TauriState<'_, State>,
    name: String,
    keys: Vec<String>,
    channel: Channel<ShortcutEvent>,
) {
    match state.lock() {
        Ok(mut inner) => {
            let shortcut = Shortcut::new(
                keys.iter().map(|k|KeyType::from(k.as_str())).collect::<Vec<KeyType>>(),
                channel,
            );
            inner.shortcuts.insert(name.clone(), shortcut);
        }
        Err(_e) => {
            // todo error
        }
    }
}

#[tauri::command]
pub fn unregister(
    state: tauri::State<'_, State>,
    name: String,
) {
    match state.lock() {
        Ok(mut inner) => {
            inner.shortcuts.remove(&name);
        }
        Err(_e) => {
            // todo error
        }
    }
}

#[tauri::command]
pub fn unregister_all(
    state: tauri::State<'_, State>,
) {
    match state.lock() {
        Ok(mut inner) => {
            inner.shortcuts.clear();
        }
        Err(_e) => {
            // todo error
        }
    }
}

#[tauri::command]
pub fn is_registered(
    state: tauri::State<'_, State>,
    name: String,
) -> bool {
    match state.lock() {
        Ok(inner) => {
            inner.shortcuts.contains_key(&name)
        }
        Err(e) => {
            panic!("lock error {}", e);
        }
    }
}