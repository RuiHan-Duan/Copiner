use copiner_workspace_service::WorkspaceCapabilities;

fn main() {
    let capabilities = WorkspaceCapabilities::default();
    debug_assert!(!capabilities.is_operational());
    println!("Copiner WorkspaceService: unconfigured; no listener or connector started");
}
