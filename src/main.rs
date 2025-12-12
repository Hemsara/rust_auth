mod app;
mod handlers;
mod routes;
mod services;
mod state;

use crate::services::env::Env;

#[tokio::main]
async fn main() {
    let app = app::create_app().await;
    let env = Env::new().expect("Failed to load environment variables");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", env.server_port))
        .await
        .unwrap();

    println!("Server running on http://localhost:{}", env.server_port);

    axum::serve(listener, app).await.unwrap();
}
