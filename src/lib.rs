use std::collections::HashMap;
use std::iter::Map;
use std::sync::{Arc, LockResult, Mutex, RwLock};
use std::thread::spawn;
use std::time::Duration;
use device_query::{mouse_state, CallbackGuard, DeviceEvents, DeviceEventsHandler, KeyboardCallback, Keycode, MouseButton};
use log::info;

use tauri::{
    ipc::Channel,
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime, State as TauriState,
};

pub use models::*;

// #[cfg(desktop)]
// mod desktop;
// #[cfg(mobile)]
// mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

// #[cfg(desktop)]
// use desktop::Hotkey;
#[cfg(mobile)]
use mobile::Hotkey;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the hotkey APIs.
// pub trait HotkeyExt<R: Runtime> {
//     fn hotkey(&self) -> &Hotkey<R>;
// }
//
// impl<R: Runtime, T: Manager<R>> crate::HotkeyExt<R> for T {
//     fn hotkey(&self) -> &Hotkey<R> {
//         self.state::<Hotkey<R>>().inner()
//     }
// }

pub struct StateInner {
    pub shortcuts: HashMap<String, Shortcut>,
}

type State = Mutex<StateInner>;

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("hotkey")
        .invoke_handler(tauri::generate_handler![
            commands::register, commands::unregister, commands::unregister_all, commands::is_registered,
        ])
        .setup(|app, _api| {
            let mut shortcuts = HashMap::new();
            // shortcuts.insert(
            //     String::from("Test 1"),
            //     Shortcut::new(vec![KeyType::Keyboard(Keycode::Command), KeyType::Keyboard(Keycode::V)])
            // );
            // shortcuts.insert(
            //     String::from("Test 2"),
            //     Shortcut::new(vec![KeyType::Keyboard(Keycode::LShift), KeyType::Mouse(3)])
            // );

            app.manage(State::new(StateInner { shortcuts }));

            let app_clone = app.clone();
            spawn(move || {
                let event_handler = DeviceEventsHandler::new(Duration::from_millis(10))
                    .expect("Could not initialize event loop");

                let app_clone1 = app_clone.clone();
                let _k_down = event_handler.on_key_down(move |key: &Keycode| {
                    let state = app_clone1.state::<State>();
                    match state.lock() {
                        Ok(mut inner) => {
                            for (name, shortcut) in inner.shortcuts.iter_mut() {
                                if shortcut.down(&KeyType::Keyboard(key.clone())) {
                                    shortcut.channel.send(ShortcutEvent { name: name.clone(), state: ShortcutState::Pressed })
                                        .expect("TODO: panic message");// TODO error
                                    println!("Shortcut {} pressed", name);
                                }
                            }
                        }
                        Err(e) => {
                            panic!("Poisoned lock at read: {:?}", e);
                        }
                    };
                });

                let app_clone2 = app_clone.clone();
                let _k_up = event_handler.on_key_up(move |key: &Keycode| {
                    let state = app_clone2.state::<State>();
                    match state.lock() {
                        Ok(mut inner) => {
                            for (name, shortcut) in inner.shortcuts.iter_mut() {
                                if shortcut.up(&KeyType::Keyboard(key.clone())) {
                                    shortcut.channel.send(ShortcutEvent { name: name.clone(), state: ShortcutState::Released })
                                        .expect("TODO: panic message");// TODO error
                                    println!("Shortcut {} released", name);
                                }
                            }
                        }
                        Err(e) => {
                            panic!("Poisoned lock at read: {:?}", e);
                        }
                    };
                });

                let app_clone3 = app_clone.clone();
                let _m_down = event_handler.on_mouse_down(move |key: &MouseButton| {
                    let state = app_clone3.state::<State>();
                    match state.lock() {
                        Ok(mut inner) => {
                            for (name, shortcut) in inner.shortcuts.iter_mut() {
                                if shortcut.down(&KeyType::Mouse(key.clone())) {
                                    shortcut.channel.send(ShortcutEvent { name: name.clone(), state: ShortcutState::Pressed })
                                        .expect("TODO: panic message");// TODO error
                                    println!("Shortcut {} pressed", name);
                                }
                            }
                        }
                        Err(e) => {
                            panic!("Poisoned lock at read: {:?}", e);
                        }
                    };
                });

                let app_clone4 = app_clone.clone();
                let _m_up = event_handler.on_mouse_up(move |key: &MouseButton| {
                    let state = app_clone4.state::<State>();
                    match state.lock() {
                        Ok(mut inner) => {
                            for (name, shortcut) in inner.shortcuts.iter_mut() {
                                if shortcut.up(&KeyType::Mouse(key.clone())) {
                                    shortcut.channel.send(ShortcutEvent { name: name.clone(), state: ShortcutState::Released })
                                        .expect("TODO: panic message");// TODO error
                                    println!("Shortcut {} released", name);
                                }
                            }
                        }
                        Err(e) => {
                            panic!("Poisoned lock at read: {:?}", e);
                        }
                    };
                });

                // Keep the main thread alive to continue receiving events
                loop {
                    std::thread::sleep(Duration::from_secs(1000));
                }
            });

            Ok(())
        })
        .build()
}
