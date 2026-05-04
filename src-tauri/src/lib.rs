use tauri::{AppHandle, Emitter};
use serde::Serialize;

#[derive(Clone, Serialize)]
struct ProgressPayload {
    percentage: u8,
}

#[tauri::command]
async fn start_work(app: AppHandle) -> Result<String, String> {
    for i in 0..=10 {
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        let pct = i * 10;
        app.emit("progress-update", ProgressPayload { percentage: pct })
            .map_err(|e| e.to_string())?;
    }
    Ok("Listo".into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![start_work])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}