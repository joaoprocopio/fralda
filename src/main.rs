use axum;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let router =
        axum::Router::<()>::new().route("/ready", axum::routing::get(async || "hello, world!"));

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
