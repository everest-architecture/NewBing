#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
async fn create_window(app: tauri::AppHandle) {
  tauri::WindowBuilder::new(&app, "label", tauri::WindowUrl::External("https://bing.com/".parse().unwrap()))
    .build()
    .unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_window])
        .setup(|app| {
            let window = tauri::WindowBuilder::new(app, "label", tauri::WindowUrl::External("https://bing.com/".parse().unwrap()))
            .title("Microsoft Bing")
            .build()
            .unwrap();
            Ok(())
          })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
