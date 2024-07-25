// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;

fn main() {
    let style: &'static str = &config::get_style();
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
