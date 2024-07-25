// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
use config::get_style;
use std::env;
use std::process;

fn print_help() {
    println!("TauriGPT");
    println!("A Tauri-based ChatGPT client");
    println!();
    println!("Usage: tauri-gpt [OPTION]");
    println!();
    println!("Options:");
    println!("    -h, --help      Print this help message");
    println!("    -v  --version   Print the version number");
    println!();
    println!("Configuration:");
    println!("    The default stylesheet is located at:");
    println!("    - Windows: %APPDATA%\\tauri-gpt\\style.css");
    println!("    - macOS: ~/.config/tauri-gpt/style.css");
    println!("    - Linux: ~/.config/tauri-gpt/style.css");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_help();
        process::exit(0);
    }
    //if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
    //    println!("{}", tauri::App::PackageInfo::version);
    //    process::exit(0);
    //}

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
                .title("TauriGPT")
                .resizable(true)
                .center()
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
