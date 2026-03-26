use axum::{
    extract::Json,
    http::StatusCode,
    response::Html,
    routing::get,
    Router,
};
use chrono::Utc;
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    timestamp: String,
    version: String,
}

async fn root() -> Html<&'static str> {
    Html(r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Rust Docker Application</title>
        <style>
            body {
                font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                max-width: 800px;
                margin: 0 auto;
                padding: 20px;
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                min-height: 100vh;
                color: white;
            }
            .container {
                background: rgba(255, 255, 255, 0.1);
                padding: 40px;
                border-radius: 15px;
                box-shadow: 0 8px 32px rgba(31, 38, 135, 0.37);
                backdrop-filter: blur(4px);
                border: 1px solid rgba(255, 255, 255, 0.18);
            }
            h1 {
                text-align: center;
                margin-bottom: 30px;
                font-size: 2.5em;
                text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
            }
            .info {
                background: rgba(255, 255, 255, 0.2);
                padding: 20px;
                border-radius: 10px;
                margin: 20px 0;
                text-align: center;
            }
            .tech-stack {
                display: flex;
                justify-content: center;
                gap: 20px;
                margin-top: 30px;
            }
            .tech-item {
                background: rgba(255, 255, 255, 0.2);
                padding: 10px 20px;
                border-radius: 25px;
                font-weight: bold;
            }
        </style>
    </head>
    <body>
        <div class="container">
            <h1>Rust Docker Application</h1>
            <p style="text-align: center; font-size: 1.2em;">Welcome to your containerized Rust web application!</p>

            <div class="info">
                <strong>Built with:</strong> Axum web framework<br>
                <strong>Runtime:</strong> Tokio async runtime<br>
                <strong>Containerized:</strong> Multi-stage Docker build<br>
                <strong>Current date:</strong> March 5, 2026
            </div>

            <div class="tech-stack">
                <div class="tech-item">Rust</div>
                <div class="tech-item">Axum</div>
                <div class="tech-item">Docker</div>
                <div class="tech-item">Tokio</div>
            </div>
        </div>
    </body>
    </html>
    "#)
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

async fn api_info() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "Rust Docker App",
        "description": "A containerized Rust web application using Axum",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": {
            "/": "HTML homepage",
            "/api/health": "Health check endpoint",
            "/api/info": "API information"
        },
        "tech_stack": ["Rust", "Axum", "Tokio", "Docker"]
    }))
}

#[tokio::main]
async fn main() {
    // Build the application with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/api/health", get(health_check))
        .route("/api/info", get(api_info));

    // Define the address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server starting on http://{}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}