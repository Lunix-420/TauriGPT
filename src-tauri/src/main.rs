use dirs;
use std::fs;
use std::path::PathBuf;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;
fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let builder = WebViewBuilder::new(&window);

    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let builder = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        WebViewBuilder::new_gtk(vbox)
    };

    // Determine the path to the CSS file based on the OS
    let css_path = match std::env::consts::OS {
        "windows" => {
            let app_data = dirs::data_dir().unwrap_or_else(|| PathBuf::from("C:\\"));
            app_data.join("tauri-gpt").join("style.css")
        }
        "macos" | "linux" => {
            let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("/"));
            config_dir.join("tauri-gpt").join("style.css")
        }
        _ => PathBuf::from("assets").join("style.css"), // Fallback for unknown OSes
    };

    // Read the CSS file contents
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
