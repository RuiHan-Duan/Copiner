mod commands;

fn main() {
    tauri::Builder::default()
        .manage(commands::LocalHarnessState::default())
        .invoke_handler(tauri::generate_handler![
            commands::runtime_status,
            commands::start_local_harness,
            commands::local_harness_snapshot,
            commands::advance_local_harness,
            commands::reset_local_harness
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Copiner desktop host");
}
