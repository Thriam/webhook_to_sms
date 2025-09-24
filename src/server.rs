// src/server.rs
use axum::{
    routing::post,
    extract::Json,
    http::StatusCode,
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::sync::oneshot;
use tracing::{info, error};

mod android;

#[derive(Deserialize)]
pub struct SmsRequest {
    pub to: String,
    pub message: String,
}

/// Start axum server listening on 0.0.0.0:port.
/// Returns when the server stops or errors.
pub async fn start_server(port: u16, shutdown_rx: oneshot::Receiver<()>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing::info!("Starting axum server on port {}", port);

    // Build routes
    let app = Router::new().route("/v1/sms", post(handle_send_sms));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            // wait for shutdown signal
            let _ = shutdown_rx.await;
            tracing::info!("Shutdown signal received for axum server");
        });

    tracing::info!("Axum server listening on {}", addr);
    if let Err(e) = server.await {
        error!("Server error: {}", e);
        return Err(Box::new(e));
    }
    Ok(())
}

async fn handle_send_sms(Json(payload): Json<SmsRequest>) -> Result<(StatusCode, String), (StatusCode, String)> {
    tracing::info!("Received SMS request to={} msg_len={}", payload.to, payload.message.len());

    // Call Android-side SMS send via JNI bridge
    match android::send_sms(&payload.to, &payload.message) {
        Ok(()) => {
            tracing::info!("SMS sent (requested)");
            Ok((StatusCode::OK, "{\"status\":\"sent\"}".to_string()))
        },
        Err(e) => {
            tracing::error!("Failed to send SMS: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{{\"error\":\"{}\"}}", e)))
        }
    }
}
