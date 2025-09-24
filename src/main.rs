// src/main.rs
use dioxus::prelude::*;
use std::sync::Arc;
use tokio::sync::oneshot;
use parking_lot::Mutex;

mod server;
mod android;

#[derive(Clone)]
struct AppState {
    server_handle: Arc<Mutex<Option<oneshot::Sender<()>>>>,
    port: Arc<Mutex<u16>>,
    status: Arc<Mutex<String>>,
}

fn main() {
    // Initialize logging for debug
    tracing_subscriber::fmt::init();

    // Start the dioxus app. On Android you must load this native lib and ensure
    // the runtime is set up per your dioxus-mobile setup; assume you integrate the compiled lib.
    dioxus::desktop::launch_cfg(app, dioxus::desktop::Config::default());
}

fn app(cx: Scope) -> Element {
    let server_handle = use_ref(cx, || None as Option<oneshot::Sender<()>>);
    let port = use_state(cx, || 8080u16);
    let status = use_state(cx, || "Server stopped".to_string());
    let enabling = use_state(cx, || false);

    let app_state = AppState {
        server_handle: Arc::new(Mutex::new(server_handle.with(|v| v.clone()))),
        port: Arc::new(Mutex::new(*port.get())),
        status: Arc::new(Mutex::new(status.get().clone())),
    };

    cx.render(rsx! {
        style { [r#"
            body { font-family: Roboto, sans-serif; margin: 16px; }
            .container { max-width: 480px; margin: 0 auto; }
            .row { display:flex; align-items:center; margin-bottom:12px; gap: 8px; }
            input[type="number"] { width: 100px; padding: 8px; }
            button { padding: 8px 12px; }
        "#] }
        div { class: "container",
            h2 { "SMS Gateway" }
            div { class: "row",
                label { "Port:" }
                input {
                    r#type: "number",
                    value: "{port}",
                    oninput: move |e| {
                        if let Ok(v) = e.value.parse::<u16>() {
                            port.set(v);
                        }
                    }
                }
            }
            div { class: "row",
                label { "Enable server:" }
                input {
                    r#type: "checkbox",
                    checked: *enabling.get(),
                    onchange: {
                        let port = port.clone();
                        let status = status.clone();
                        let server_handle = server_handle.clone();
                        move |e| {
                            let enabled = e.value == "on";
                            enabling.set(enabled);
                            let port_val = *port.get();
                            if enabled {
                                // Start server
                                let (tx, rx) = oneshot::channel::<()>();
                                *server_handle.write() = Some(tx);
                                status.set(format!("Starting server on 0.0.0.0:{} ...", port_val));

                                // spawn tokio runtime task to start server
                                // use a new tokio runtime thread to run axum server (since Dioxus desktop has its own)
                                std::thread::spawn(move || {
                                    // Start tokio runtime
                                    let rt = tokio::runtime::Builder::new_current_thread()
                                        .enable_all()
                                        .build()
                                        .unwrap();
                                    rt.block_on(async move {
                                        if let Err(e) = server::start_server(port_val, rx).await {
                                            tracing::error!("Server failed: {}", e);
                                        }
                                    });
                                });

                                status.set(format!("Server running on port {}", port_val));
                            } else {
                                // Stop server
                                if let Some(tx) = server_handle.read().clone() {
                                    let _ = tx.send(());
                                    *server_handle.write() = None;
                                }
                                status.set("Server stopped".to_string());
                            }
                        }
                    }
                }
            }
            div { class: "row",
                p { strong { "Status: " } "{status}" }
            }
            p { "Notes: Android runtime must call NativeBridge.init(activity) to provide Context to Rust/JNI." }
        }
    })
}
