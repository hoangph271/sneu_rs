#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(debug_assertions)]
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            #[cfg(debug_assertions)]
            _app.get_window("main")
                .unwrap_or_else(|| panic!("get_window(\"main\") failed"))
                .open_devtools();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
