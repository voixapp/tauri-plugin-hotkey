const COMMANDS: &[&str] = &["register", "unregister", "unregister_all", "is_registered"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .build();
}
