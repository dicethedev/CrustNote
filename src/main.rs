mod task;
mod handlers;
mod routes;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use handlers::Db;
use routes::create_router;

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(Mutex::new(Vec::new()));

    let app = create_router(db);

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    println!("🚀 Running CrustNote API on http://{}", addr);

    // Use axum::serve with a TcpListener
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
