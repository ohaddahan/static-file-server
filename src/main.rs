mod cli;
mod server;
mod terminal;

use crate::cli::CliArgs;
use crate::server::create_routes;
use crate::terminal::colored_msg;
use axum::Router;
use clap::Parser;
use std::fs;
use std::net::SocketAddr;
use std::process::exit;
use termcolor::Color;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "static_file_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = CliArgs::parse();
    if fs::metadata(&args.dir).is_err() {
        colored_msg(&format!("Dir {} does not exist", args.dir), Color::Red).unwrap_or_default();
        exit(1);
    } else {
        colored_msg(&format!("Found dir {}", args.dir), Color::Green).unwrap_or_default();
    }
    let routes = create_routes(&args);
    tokio::join!(serve(routes, &args),);
}

async fn serve(app: Router, args: &CliArgs) {
    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    colored_msg(
        &format!(
            "Listening on = {}, server dir = {}, cors enabled = {}, compression enabled = {}",
            listener.local_addr().unwrap(),
            &args.dir,
            &args.cors,
            &args.compression
        ),
        Color::Green,
    )
    .unwrap_or_default();
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}
