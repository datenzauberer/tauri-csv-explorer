use std::sync::Mutex;

use clap::Parser;
use csv_explorer::{Cli, read_csv_file_as_json_as_string};
use tauri::State;

// REQ-003, REQ-007 File Association: AppState contains Cli Filename
struct AppState {
    filename: Option<String>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let cli = Cli::parse();
    let filename = cli.filename;

    tauri::Builder::default()
        .manage(Mutex::new(AppState { filename }))
        .setup(|app| {
            // REQ-100 Check if update is available
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                update(handle).await.unwrap();
            });
            Ok(())
        })
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_cli_filename,
            tauri_read_csv_file
        ])
        // REQ-007 Mac File Association
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            #[cfg(target_os = "macos")]
            // MacOS specific event listener for when the app is opened with a filename
            if let tauri::RunEvent::Opened { urls } = event {
                use tauri::Manager;

                let filename = urls
                    .first()
                    .unwrap()
                    .to_file_path()
                    .unwrap()
                    .as_path()
                    .display()
                    .to_string();
                let state = app.state::<Mutex<AppState>>();
                state.lock().expect("could not lock state").filename = Some(filename.clone());
            }
        })
}

#[tauri::command]
fn get_cli_filename(state: State<Mutex<AppState>>) -> Option<String> {
    state.lock().expect("could not lock state").filename.clone()
}

#[tauri::command]
fn tauri_read_csv_file(path: &str) -> Result<String, String> {
    read_csv_file_as_json_as_string(path)
        .map_err(move |e| format!("Failed to read file from path '{}': {}", path, e))
}

// REQ-100 Check if update is available
async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    use tauri_plugin_updater::UpdaterExt;
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        // alternatively we could also call update.download() and update.install() separately
        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    println!("download finished");
                },
            )
            .await?;

        println!("update installed");
        app.restart();
    } else {
        println!("No update available.");
    }

    Ok(())
}
