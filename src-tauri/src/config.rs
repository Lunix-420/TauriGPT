use dirs;
use std::fs;
use std::path::PathBuf;

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
pub fn get_style() -> &'static str {
    let path = get_config_path();
    let style_path = path.join("style.css");
    let style = match fs::read_to_string(style_path) {
        Ok(content) => content,
        Err(_) => "".to_string(),
    };
    return Box::leak(style.into_boxed_str());
}
