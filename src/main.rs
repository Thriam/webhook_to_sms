use dioxus::prelude::*;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

mod server;
mod android;

fn main() {
    dioxus_mobile::launch(app);
}

fn app(cx: Scope) -> Element {
    let port = use_state(&cx, || "8080".to_string());
    let ws_path = use_state(&cx, || "/v1/ws".to_string());
    let running = use_state(&cx, || false);
    let status = use_state(&cx, || "".to_string());

    let rt = Runtime::new().unwrap();
    let server_handle = use_ref(&cx, || Arc::new(Mutex::new(None)));

    cx.render(rsx! {
        div {
            h1 { "📱 SMS Gateway" }

            input {
                value: "{port}",
                oninput: move |e| port.set(e.value.clone()),
                placeholder: "Port"
            }

            input {
                value: "{ws_path}",
                oninput: move |e| ws_path.set(e.value.clone()),
                placeholder: "WebSocket Path (e.g. /v1/ws)"
            }

            button {
                onclick: move |_| {
                    if *running.get() {
                        *server_handle.read().lock().unwrap() = None;
                        running.set(false);
                        status.set("Server stopped".into());
                    } else {
                        let p = port.get().clone();
                        let w = ws_path.get().clone();
                        let handle = rt.spawn(async move {
                            server::start_server(p.parse().unwrap(), w).await;
                        });
                        *server_handle.read().lock().unwrap() = Some(handle);
                        running.set(true);
                        status.set(format!("Server running on 0.0.0.0:{}{}", port.get(), ws_path.get()));
                    }
                },
                if *running.get() { "Stop Server" } else { "Start Server" }
            }

            p { "{status}" }
        }
    })
}
