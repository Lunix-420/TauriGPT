// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
// import dir
use dirs;

fn get_config_path() -> PathBuf {
    match std::env::consts::OS {
        "windows" => {
            let app_data = dirs::data_dir().unwrap_or_else(|| PathBuf::from("C:\\"));
            return app_data.join("tauri-gpt");
        }
        "macos" | "linux" => {
            let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("/"));
            return config_dir.join("tauri-gpt");
        }
        _ => return PathBuf::from("assets").join("style.css"),
    };
}
fn get_style() -> &'static str {
    let path = get_config_path();
    let style_path = path.join("style.css");
    let style = match fs::read_to_string(style_path) {
        Ok(content) => content,
        Err(_) => "".to_string(),
    };
    return Box::leak(style.into_boxed_str());
}

fn main() {
    let style: &'static str = &get_style();
    let url = tauri::Url::parse("http://chatgpt.com").unwrap();
    tauri::Builder::default()
        .setup(move |app| {
            tauri::WindowBuilder::new(app, "TauriGPT", tauri::WindowUrl::External(url))
                .initialization_script(&format!(
                    r#"
                        window.addEventListener('load', () => {{
                            const style = document.createElement('style');
                            style.innerHTML = `{}`;
                            document.head.appendChild(style);
                        }});
                        "#,
                    style
                ))
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
