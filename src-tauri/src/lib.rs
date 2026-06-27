//! Thin Tauri glue over `regatta_core`. Logic belongs in the pure core; this only wires it.

/// Returns a worktree-safe slug for a session name, computed by the pure core.
#[tauri::command]
fn slugify(name: &str) -> String {
    regatta_core::slug::slugify(name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![slugify])
        .run(tauri::generate_context!())
        .expect("error while running Regatta");
}
