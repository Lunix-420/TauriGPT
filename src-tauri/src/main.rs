// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

const INIT_SCRIPT: &str = r#"
    console.log("hello world from js init script");
"#;

fn main() {
    let url = tauri::Url::parse("http://chatgpt.com").unwrap();
    tauri::Builder::default()
        .setup(|app| {
            let window =
                tauri::WindowBuilder::new(app, "TauriGPT", tauri::WindowUrl::External(url))
                    .initialization_script(INIT_SCRIPT)
                    .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
