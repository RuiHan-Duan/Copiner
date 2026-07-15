#[tauri::command]
pub fn runtime_status() -> &'static str {
    "unconfigured"
}
