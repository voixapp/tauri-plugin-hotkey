import { invoke, Channel } from '@tauri-apps/api/core'

export interface ShortcutEvent {
  name: string
  state: 'Released' | 'Pressed'
}

export type ShortcutHandler = (event: ShortcutEvent) => void

/**
 * Register shortcut.
 *
 * The handler is called when the registered shortcut is pressed by the user.
 *
 * @example
 * ```typescript
 * import { register } from '@tauri-apps/plugin-hotkey';
 *
 * // register hotkey
 * await register('MyShortcut1', ['LShift', 'Z'], (event) => {
 *   if (event.state === "Pressed") {
 *       console.log('Shortcut triggered');
 *   }
 * });
 * ```
 *
 * @param name Shortcut name/ID
 * @param keys Shortcut keys as array of strings ['LShift', 'Z']
 * @param handler Shortcut handler callback - takes the triggered shortcut as argument
 */
async function register(
    name: string,
    keys: string[],
    handler: ShortcutHandler
): Promise<void> {
  const channel = new Channel<ShortcutEvent>()
  channel.onmessage = handler

  return await invoke('plugin:hotkey|register', {
    name: name,
    keys: keys,
    channel: channel,
  })
}

/**
 * Unregister shortcut.
 *
 * @example
 * ```typescript
 * import { unregister } from '@tauri-apps/plugin-hotkey';
 *
 * // unregister hotkey
 * await unregister('MyShortcut1');
 * ```
 *
 * @param name Shortcut name/ID
 */
async function unregister(name: string): Promise<void> {
  return await invoke('plugin:hotkey|unregister', {
    name: name,
  })
}

/**
 * Unregister all shortcuts.
 *
 * @example
 * ```typescript
 * import { unregisterAll } from '@tauri-apps/plugin-hotkey';
 * await unregisterAll();
 * ```
 */
async function unregisterAll(): Promise<void> {
  return await invoke('plugin:hotkey|unregister_all', {})
}

/**
 * Determines whether the given shortcut is registered by this application or not.
 *
 * @example
 * ```typescript
 * import { isRegistered } from '@tauri-apps/plugin-hotkey';
 * const isRegistered = await isRegistered('MyShortcut1');
 * ```
 *
 * @param name Shortcut name/ID
 *
 * @since 2.0.0
 */
async function isRegistered(name: string): Promise<boolean> {
  return await invoke('plugin:hotkey|is_registered', {
    name
  })
}

export { register, unregister, unregisterAll, isRegistered }
