use axum::{
    extract::{Json, ws::{WebSocketUpgrade, Message}},
    response::IntoResponse,
    routing::{post, get},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use futures_util::StreamExt;
use tokio_stream::wrappers::BroadcastStream;

use crate::android;

#[derive(Deserialize)]
pub struct SmsRequest {
    to: String,
    message: String,
}

async fn send_sms(Json(payload): Json<SmsRequest>) -> &'static str {
    android::send_sms(&payload.to, &payload.message);
    "SMS sent"
}

async fn sms_ws(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        let mut rx = android::subscribe_sms();
        let mut stream = BroadcastStream::new(rx);

        while let Some(Ok((from, body))) = stream.next().await {
            let json = serde_json::json!({
                "from": from,
                "message": body
            });
            if socket.send(Message::Text(json.to_string())).await.is_err() {
                break;
            }
        }
    })
}

pub async fn start_server(port: u16, ws_path: String) {
    let app = Router::new()
        .route("/v1/sms", post(send_sms))
        .route(&ws_path, get(sms_ws)); // <-- user defined path

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
