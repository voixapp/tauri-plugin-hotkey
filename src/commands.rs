use crate::models::*;
use crate::State;
use tauri::{
    ipc::Channel,
    plugin::{Builder as PluginBuilder, TauriPlugin},
    AppHandle, Manager, Runtime, State as TauriState,
};

#[tauri::command]
pub fn register/*<R: Runtime>*/(
    // app: AppHandle<R>,
    state: TauriState<'_, State>,
    name: String,
    keys: Vec<String>,
    channel: Channel<ShortcutEvent>,
) {
    // let mut state = app.state::<State>();
    match state.lock() {
        Ok(mut inner) => {
            let shortcut = Shortcut::new(
                keys.iter().map(|k|KeyType::from(k.as_str())).collect::<Vec<KeyType>>(),
                channel,
            );
            inner.shortcuts.insert(name.clone(), shortcut);
        }
        Err(e) => {
            // todo error
        }
    }
}

#[tauri::command]
pub fn unregister/*<R: Runtime>*/(
    // app: AppHandle<R>,
    state: tauri::State<'_, State>,
    name: String,
) {
    // let mut state = app.state::<State>();
    match state.lock() {
        Ok(mut inner) => {
            inner.shortcuts.remove(&name);
        }
        Err(e) => {
            // todo error
        }
    }
}

#[tauri::command]
pub fn unregister_all/*<R: Runtime>*/(
    state: tauri::State<'_, State>,
) {
    match state.lock() {
        Ok(mut inner) => {
            inner.shortcuts.clear();
        }
        Err(e) => {
            // todo error
        }
    }
}

#[tauri::command]
pub fn is_registered/*<R: Runtime>*/(
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