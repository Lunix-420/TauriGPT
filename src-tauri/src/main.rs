use dirs;
use std::fs;
use std::path::PathBuf;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

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

// Tauri based ChatGPT desktop app
fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("TauriGPT");

    #[cfg(not(target_os = "linux"))]
    let builder = WebViewBuilder::new(&window);

    #[cfg(target_os = "linux")]
    let builder = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        WebViewBuilder::new_gtk(vbox)
    };

    // Determine the path to the CSS file based on the OS

    // Read the CSS file contents
    let css_path = get_config_path().join("style.css");
    let css_content = fs::read_to_string(&css_path)
        .unwrap_or_else(|_| panic!("CSS file not found at: {}", css_path.display()));

    // Create the WebView with the CSS injection
    let _webview = builder
        .with_url("http://chatgpt.com")
        .with_initialization_script(&format!(
            r#"
            window.addEventListener('load', () => {{
                const style = document.createElement('style');
                style.innerHTML = `{}`;
                document.head.appendChild(style);
            }});
            "#,
            css_content
        ))
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        }
    })
}
