use axum::{routing::get, Router};
use std::net::{Ipv4Addr, SocketAddr};

async fn home() -> &'static str {
    "Hello from home"
}

async fn about() -> &'static str {
    "Hello from about"
}

async fn contact() -> &'static str {
    "Hello from contact"
}

#[tokio::main]
async fn main() {
    println!("Hello world");

    let app = Router::new()
        .route("/", get(home))
        .route("/about", get(about))
        .route("/contacts", get(contact));

    let addr = SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), 8080));
    println!("Listening: {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();


}
