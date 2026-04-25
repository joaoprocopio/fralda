use anyhow::Result;
use axum;
use fralda::ext;
use std::process;
use tokio::net::TcpListener;

fn main() {
    ext::tracing::init_tracing();
    let runtime = ext::tokio::new_runtime();

    // runtime.handle().clone()
    runtime.block_on(run()).unwrap_or_else(|err| {
        tracing::error!("server boot failed with: {}", err);
        process::exit(1)
    })
}

async fn run() -> Result<()> {
    let router =
        axum::Router::<()>::new().route("/ready", axum::routing::get(async || "hello, world!"));

    let listener = TcpListener::bind("0.0.0.0:8000").await?;

    axum::serve(listener, router).await?;

    Ok(())
}
