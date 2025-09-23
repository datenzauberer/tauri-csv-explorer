use csv_explorer::read_csv_file_as_json_as_string;

#[tauri::command]
#[cfg(not(target_os = "android"))]
fn tauri_read_csv_file(path: &str) -> Result<String, String> {
    read_csv_file_as_json_as_string(path)
    .map_err(move |e| format!("Failed to read file from path '{}': {}", path, e.to_string()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![tauri_read_csv_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
