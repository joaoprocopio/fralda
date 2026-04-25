use anyhow::Result;
use axum::{self, http::StatusCode};
use std::process;
use tokio::{
    net::TcpListener,
    runtime,
    signal::unix::{SignalKind, signal},
};
use tower_http::trace::TraceLayer;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_line_number(true)
                .with_file(true),
        )
        .init();
}

pub fn new_runtime() -> runtime::Runtime {
    match runtime::Builder::new_current_thread().enable_all().build() {
        Ok(rt) => rt,
        Err(err) => {
            tracing::error!("tokio runtime boot failed: {err}");
            std::process::exit(1);
        }
    }
}

pub async fn shutdown_signal() -> Result<impl Future<Output = ()>> {
    let mut terminate = signal(SignalKind::terminate())?;
    let mut interrupt = signal(SignalKind::interrupt())?;

    let signal = async move {
        tokio::select! {
            Some(_) = terminate.recv() => {
                tracing::info!("SIGTERM received");
            },
            Some(_) = interrupt.recv() => {
                tracing::info!("SIGINT received");
            }
        }
    };

    Ok(signal)
}

fn main() {
    init_tracing();
    let runtime = new_runtime();

    runtime
        .block_on(run(/* runtime.handle().clone() */))
        .unwrap_or_else(|err| {
            tracing::error!("server boot failed with: {}", err);
            process::exit(1)
        })
}

async fn run() -> Result<()> {
    let router = axum::Router::<()>::new()
        .route("/ready", axum::routing::get(ready))
        .route("/fraud-score", axum::routing::post(fraud_score))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:8000").await?;

    axum::serve(listener, router).await?;

    Ok(())
}

async fn ready() -> StatusCode {
    StatusCode::OK
}

async fn fraud_score() -> StatusCode {
    StatusCode::CREATED
}
