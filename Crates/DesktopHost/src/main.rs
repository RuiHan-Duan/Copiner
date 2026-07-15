mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::runtime_status])
        .run(tauri::generate_context!())
        .expect("failed to run Copiner desktop host");
}
